# serde_json

# Serde JSON

JSON is a ubiquitous open-standard format that uses human-readable text to
transmit data objects consisting of key-value pairs.

```json
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}
```

There are three common ways that you might find yourself needing to work
with JSON data in Rust.

 - **As text data.** An unprocessed string of JSON data that you receive on
   an HTTP endpoint, read from a file, or prepare to send to a remote
   server.
 - **As an untyped or loosely typed representation.** Maybe you want to
   check that some JSON data is valid before passing it on, but without
   knowing the structure of what it contains. Or you want to do very basic
   manipulations like insert a key in a particular spot.
 - **As a strongly typed Rust data structure.** When you expect all or most
   of your data to conform to a particular structure and want to get real
   work done without JSON's loosey-goosey nature tripping you up.

Serde JSON provides efficient, flexible, safe ways of converting data
between each of these representations.

# Operating on untyped JSON values

Any valid JSON data can be manipulated in the following recursive enum
representation. This data structure is [`serde_json::Value`][value].

```
# use serde_json::{Number, Map};
#
# #[allow(dead_code)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`][from_str] function. There is also [`from_slice`]
for parsing from a byte slice `&[u8]` and [`from_reader`] for parsing from
any `io::Read` like a File or a TCP stream.

```
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
#
# fn main() {
#     untyped_example().unwrap();
# }
```

The result of square bracket indexing like `v["name"]` is a borrow of the
data at that index, so the type is `&Value`. A JSON map can be indexed with
string keys, while a JSON array can be indexed with integer keys. If the
type of the data is not right for the type with which it is being indexed,
or if a map does not contain the key being indexed, or if the index into a
vector is out of bounds, the returned element is `Value::Null`.

When a `Value` is printed, it is printed as a JSON string. So in the code
above, the output looks like `Please call "John Doe" at the number "+44
1234567"`. The quotation marks appear because `v["name"]` is a `&Value`
containing a JSON string and its JSON representation is `"John Doe"`.
Printing as a plain string without quotation marks involves converting from
a JSON string to a Rust string with [`as_str()`] or avoiding the use of
`Value` as described in the following section.

[`as_str()`]: crate::Value::as_str

The `Value` representation is sufficient for very basic tasks but can be
tedious to work with for anything more significant. Error handling is
verbose to implement correctly, for example imagine trying to detect the
presence of unrecognized fields in the input data. The compiler is powerless
to help you when you make a mistake, for example imagine typoing `v["name"]`
as `v["nmae"]` in one of the dozens of places it is used in your code.

# Parsing JSON as strongly typed data structures

Serde provides a powerful way of mapping JSON data into Rust data structures
largely automatically.

```
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
#
# fn main() {
#     typed_example().unwrap();
# }
```

This is the same `serde_json::from_str` function as before, but this time we
assign the return value to a variable of type `Person` so Serde will
automatically interpret the input data as a `Person` and produce informative
error messages if the layout does not conform to what a `Person` is expected
to look like.

Any type that implements Serde's `Deserialize` trait can be deserialized
this way. This includes built-in Rust standard library types like `Vec<T>`
and `HashMap<K, V>`, as well as any structs or enums annotated with
`#[derive(Deserialize)]`.

Once we have `p` of type `Person`, our IDE and the Rust compiler can help us
use it correctly like they do for any other Rust code. The IDE can
autocomplete field names to prevent typos, which was impossible in the
`serde_json::Value` representation. And the Rust compiler can check that
when we write `p.phones[0]`, then `p.phones` is guaranteed to be a
`Vec<String>` so indexing into it makes sense and produces a `String`.

# Constructing JSON values

Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
objects with very natural JSON syntax.

```
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```
# use serde_json::json;
#
# fn random_phone() -> u16 { 0 }
#
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

This is amazingly convenient, but we have the problem we had before with
`Value`: the IDE and Rust compiler cannot help us if we get it wrong. Serde
JSON provides a better way of serializing strongly-typed data structures
into JSON text.

# Creating JSON by serializing data structures

A data structure can be converted to a JSON string by
[`serde_json::to_string`][to_string]. There is also
[`serde_json::to_vec`][to_vec] which serializes to a `Vec<u8>` and
[`serde_json::to_writer`][to_writer] which serializes to any `io::Write`
such as a File or a TCP stream.

```
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}
#
# fn main() {
#     print_an_address().unwrap();
# }
```

Any type that implements Serde's `Serialize` trait can be serialized this
way. This includes built-in Rust standard library types like `Vec<T>` and
`HashMap<K, V>`, as well as any structs or enums annotated with
`#[derive(Serialize)]`.

# No-std support

As long as there is a memory allocator, it is possible to use serde_json
without the rest of the Rust standard library. Disable the default "std"
feature and enable the "alloc" feature:

```toml
[dependencies]
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

For JSON support in Serde without a memory allocator, please see the
[`serde-json-core`] crate.

[value]: crate::value::Value
[from_str]: crate::de::from_str
[from_slice]: crate::de::from_slice
[from_reader]: crate::de::from_reader
[to_string]: crate::ser::to_string
[to_vec]: crate::ser::to_vec
[to_writer]: crate::ser::to_writer
[macro]: crate::json
[`serde-json-core`]: https://github.com/rust-embedded-community/serde-json-core

## Table of Contents

- **serde_json**
  - [de](#serde_json-de)
  - [error](#serde_json-error)
  - [map](#serde_json-map)
  - [ser](#serde_json-ser)
  - [value](#serde_json-value)
- **de**
  - [Deserializer](#serde_json-de-deserializer)
  - [StreamDeserializer](#serde_json-de-streamdeserializer)
  - [from_reader](#serde_json-de-from_reader)
  - [from_slice](#serde_json-de-from_slice)
  - [from_str](#serde_json-de-from_str)
- **error**
  - [Category](#serde_json-error-category)
  - [Error](#serde_json-error-error)
  - [Result](#serde_json-error-result)
- **map**
  - [Entry](#serde_json-map-entry)
  - [IntoIter](#serde_json-map-intoiter)
  - [IntoValues](#serde_json-map-intovalues)
  - [Iter](#serde_json-map-iter)
  - [IterMut](#serde_json-map-itermut)
  - [Keys](#serde_json-map-keys)
  - [Map](#serde_json-map-map)
  - [OccupiedEntry](#serde_json-map-occupiedentry)
  - [VacantEntry](#serde_json-map-vacantentry)
  - [Values](#serde_json-map-values)
  - [ValuesMut](#serde_json-map-valuesmut)
- **number**
  - [Number](#serde_json-number-number)
- **read**
  - [Fused](#serde_json-read-fused)
  - [IoRead](#serde_json-read-ioread)
  - [Position](#serde_json-read-position)
  - [Read](#serde_json-read-read)
  - [Reference](#serde_json-read-reference)
  - [SliceRead](#serde_json-read-sliceread)
  - [StrRead](#serde_json-read-strread)
- **read::private**
  - [Sealed](#serde_json-read-private-sealed)
- **ser**
  - [CharEscape](#serde_json-ser-charescape)
  - [CompactFormatter](#serde_json-ser-compactformatter)
  - [Formatter](#serde_json-ser-formatter)
  - [PrettyFormatter](#serde_json-ser-prettyformatter)
  - [Serializer](#serde_json-ser-serializer)
  - [to_string](#serde_json-ser-to_string)
  - [to_string_pretty](#serde_json-ser-to_string_pretty)
  - [to_vec](#serde_json-ser-to_vec)
  - [to_vec_pretty](#serde_json-ser-to_vec_pretty)
  - [to_writer](#serde_json-ser-to_writer)
  - [to_writer_pretty](#serde_json-ser-to_writer_pretty)
- **value**
  - [Value](#serde_json-value-value)
  - [from_value](#serde_json-value-from_value)
  - [to_value](#serde_json-value-to_value)
- **value::index**
  - [Index](#serde_json-value-index-index)
- **value::index::private**
  - [Sealed](#serde_json-value-index-private-sealed)
- **value::ser**
  - [SerializeMap](#serde_json-value-ser-serializemap)
  - [SerializeStructVariant](#serde_json-value-ser-serializestructvariant)
  - [SerializeTupleVariant](#serde_json-value-ser-serializetuplevariant)
  - [SerializeVec](#serde_json-value-ser-serializevec)
  - [Serializer](#serde_json-value-ser-serializer)


---

# Module: `serde_json`

## Module: de

Deserialize JSON data to a Rust data structure.



## Module: error

When serializing or deserializing JSON goes wrong.



## Module: map

A map of String to serde_json::Value.

By default the map is backed by a [`BTreeMap`]. Enable the `preserve_order`
feature of serde_json to use [`IndexMap`] instead.

[`BTreeMap`]: std::collections::BTreeMap
[`IndexMap`]: indexmap::IndexMap



## Module: ser

Serialize a Rust data structure into JSON data.



## Module: value

The Value enum, a loosely typed way of representing any valid JSON value.

# Constructing JSON

Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
objects with very natural JSON syntax.

```
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```
# use serde_json::json;
#
# fn random_phone() -> u16 { 0 }
#
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`][from_str] function. There is also
[`from_slice`][from_slice] for parsing from a byte slice `&[u8]` and
[`from_reader`][from_reader] for parsing from any `io::Read` like a File or
a TCP stream.

```
use serde_json::{json, Value, Error};

fn untyped_example() -> Result<(), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
#
# untyped_example().unwrap();
```

[macro]: crate::json
[from_str]: crate::de::from_str
[from_slice]: crate::de::from_slice
[from_reader]: crate::de::from_reader



---

# Module: `serde_json::de`

## serde_json::de::Deserializer

**Type:** Struct

A structure that deserializes JSON into Rust values.

**Generic Parameters:**
- R

**Methods:**

- `fn from_reader(reader: R) -> Self` - Creates a JSON deserializer from an `io::Read`.
- `fn end(self: &'_ mut Self) -> Result` - The `Deserializer::end` method should be called after a value has been fully deserialized.
- `fn into_iter<T>(self: Self) -> StreamDeserializer` - Turn a JSON deserializer into an iterator over values of type T.
- `fn from_slice(bytes: &'a [u8]) -> Self` - Creates a JSON deserializer from a `&[u8]`.
- `fn new(read: R) -> Self` - Create a JSON deserializer from one of the possible serde_json input
- `fn from_str(s: &'a str) -> Self` - Creates a JSON deserializer from a `&str`.



## serde_json::de::StreamDeserializer

**Type:** Struct

Iterator that deserializes a stream into multiple JSON values.

A stream deserializer can be created from any JSON deserializer using the
`Deserializer::into_iter` method.

The data can consist of any JSON value. Values need to be a self-delineating value e.g.
arrays, objects, or strings, or be followed by whitespace or a self-delineating value.

```
use serde_json::{Deserializer, Value};

fn main() {
    let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";

    let stream = Deserializer::from_str(data).into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }
}
```

**Generic Parameters:**
- ''de
- R
- T

**Methods:**

- `fn new(read: R) -> Self` - Create a JSON stream deserializer from one of the possible serde_json
- `fn byte_offset(self: &'_ Self) -> usize` - Returns the number of bytes so far deserialized into a successful `T`.

**Trait Implementations:**

- **FusedIterator**
- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 



## serde_json::de::from_reader

**Type:** Function

Deserialize an instance of type `T` from an I/O stream of JSON.

The content of the I/O stream is deserialized directly from the stream
without being buffered in memory by serde_json.

When reading from a source against which short reads are not efficient, such
as a [`File`], you will want to apply your own buffering because serde_json
will not buffer the input. See [`std::io::BufReader`].

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own [`Deserializer`].

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying [`from_str`]
or [`from_slice`] on it. See [issue #160].

[`File`]: std::fs::File
[issue #160]: https://github.com/serde-rs/json/issues/160

# Example

Reading the contents of a file.

```
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
# }
# fn fake_main() {
    let u = read_user_from_file("test.json").unwrap();
    println!("{:#?}", u);
}
```

Reading from a persistent socket connection.

```
use serde::Deserialize;

use std::error::Error;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_stream(stream: &mut BufReader<TcpStream>) -> Result<User, Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let u = User::deserialize(&mut de)?;

    Ok(u)
}

fn main() {
# }
# fn fake_main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for tcp_stream in listener.incoming() {
        let mut buffered = BufReader::new(tcp_stream.unwrap());
        println!("{:#?}", read_user_from_stream(&mut buffered));
    }
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_reader<R, T>(rdr: R) -> crate::error::Result
```



## serde_json::de::from_slice

**Type:** Function

Deserialize an instance of type `T` from bytes of JSON text.

# Example

```
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&[u8]`
    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_slice(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_slice<''a, T>(v: &'a [u8]) -> crate::error::Result
```



## serde_json::de::from_str

**Type:** Function

Deserialize an instance of type `T` from a string of JSON text.

# Example

```
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_str<''a, T>(s: &'a str) -> crate::error::Result
```



---

# Module: `serde_json::error`

## serde_json::error::Category

**Type:** Enum

Categorizes the cause of a `serde_json::Error`.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Io` | Unit | The error was caused by a failure to read or write bytes on an I/O |
| `Syntax` | Unit | The error was caused by input that was not syntactically valid JSON. |
| `Data` | Unit | The error was caused by input data that was semantically incorrect. |
| `Eof` | Unit | The error was caused by prematurely reaching the end of the input data. |

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Category) -> bool` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Category` - 
- **Eq**
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **StructuralPartialEq**
- **Copy**



## serde_json::error::Error

**Type:** Struct

This type represents all possible errors that can occur when serializing or
deserializing JSON data.

**Methods:**

- `fn line(self: &'_ Self) -> usize` - One-based line number at which the error was detected.
- `fn column(self: &'_ Self) -> usize` - One-based column number at which the error was detected.
- `fn classify(self: &'_ Self) -> Category` - Categorizes the cause of this error.
- `fn is_io(self: &'_ Self) -> bool` - Returns true if this error was caused by a failure to read or write
- `fn is_syntax(self: &'_ Self) -> bool` - Returns true if this error was caused by input that was not
- `fn is_data(self: &'_ Self) -> bool` - Returns true if this error was caused by input data that was
- `fn is_eof(self: &'_ Self) -> bool` - Returns true if this error was caused by prematurely reaching the end of
- `fn io_error_kind(self: &'_ Self) -> Option` - The kind reported by the underlying standard library I/O error, if this

**Trait Implementations:**

- **Error**
  - `fn custom<T>(msg: T) -> Error` - 
- **Display**
  - `fn fmt(self: &'_ Self, f: &'_ mut fmt::Formatter) -> fmt::Result` - 
- **Error**
  - `fn source(self: &'_ Self) -> Option` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut fmt::Formatter) -> fmt::Result` - 
- **Error**
  - `fn custom<T>(msg: T) -> Error` - 
  - `fn invalid_type(unexp: de::Unexpected, exp: &'_ dyn de::Expected) -> Self` - 
  - `fn invalid_value(unexp: de::Unexpected, exp: &'_ dyn de::Expected) -> Self` - 



## serde_json::error::Result

**Type:** Type Alias

Alias for a `Result` with the error type `serde_json::Error`.



---

# Module: `serde_json::map`

## serde_json::map::Entry

**Type:** Enum

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the [`entry`] method on [`Map`].

[`entry`]: Map::entry

**Generic Parameters:**
- ''a

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Vacant` | Tuple(VacantEntry) | A vacant Entry. |
| `Occupied` | Tuple(OccupiedEntry) | An occupied Entry. |

**Methods:**

- `fn key(self: &'_ Self) -> &'_ String` - Returns a reference to this entry's key.
- `fn or_insert(self: Self, default: Value) -> &'a mut Value` - Ensures a value is in the entry by inserting the default if empty, and
- `fn or_insert_with<F>(self: Self, default: F) -> &'a mut Value` - Ensures a value is in the entry by inserting the result of the default
- `fn and_modify<F>(self: Self, f: F) -> Self` - Provides in-place mutable access to an occupied entry before any



## serde_json::map::IntoIter

**Type:** Struct

An owning iterator over a serde_json::Map's entries.

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &'_ Self) -> usize` - 
- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 
  - `fn size_hint(self: &'_ Self) -> (usize, Option)` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **DoubleEndedIterator**
  - `fn next_back(self: &'_ mut Self) -> Option` - 
- **FusedIterator**



## serde_json::map::IntoValues

**Type:** Struct

An owning iterator over a serde_json::Map's values.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 
  - `fn size_hint(self: &'_ Self) -> (usize, Option)` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **DoubleEndedIterator**
  - `fn next_back(self: &'_ mut Self) -> Option` - 
- **FusedIterator**
- **ExactSizeIterator**
  - `fn len(self: &'_ Self) -> usize` - 



## serde_json::map::Iter

**Type:** Struct

An iterator over a serde_json::Map's entries.

**Generic Parameters:**
- ''a

**Trait Implementations:**

- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 
  - `fn size_hint(self: &'_ Self) -> (usize, Option)` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **FusedIterator**
- **DoubleEndedIterator**
  - `fn next_back(self: &'_ mut Self) -> Option` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Iter` - 
- **ExactSizeIterator**
  - `fn len(self: &'_ Self) -> usize` - 



## serde_json::map::IterMut

**Type:** Struct

A mutable iterator over a serde_json::Map's entries.

**Generic Parameters:**
- ''a

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: &'_ mut Self) -> Option` - 
- **FusedIterator**
- **ExactSizeIterator**
  - `fn len(self: &'_ Self) -> usize` - 
- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 
  - `fn size_hint(self: &'_ Self) -> (usize, Option)` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 



## serde_json::map::Keys

**Type:** Struct

An iterator over a serde_json::Map's keys.

**Generic Parameters:**
- ''a

**Trait Implementations:**

- **FusedIterator**
- **ExactSizeIterator**
  - `fn len(self: &'_ Self) -> usize` - 
- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 
  - `fn size_hint(self: &'_ Self) -> (usize, Option)` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **DoubleEndedIterator**
  - `fn next_back(self: &'_ mut Self) -> Option` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Keys` - 



## serde_json::map::Map

**Type:** Struct

Represents a JSON key/value type.

**Generic Parameters:**
- K
- V

**Methods:**

- `fn new() -> Self` - Makes a new empty Map.
- `fn with_capacity(capacity: usize) -> Self` - Makes a new empty Map with the given initial capacity.
- `fn clear(self: &'_ mut Self)` - Clears the map, removing all values.
- `fn get<Q>(self: &'_ Self, key: &'_ Q) -> Option` - Returns a reference to the value corresponding to the key.
- `fn contains_key<Q>(self: &'_ Self, key: &'_ Q) -> bool` - Returns true if the map contains a value for the specified key.
- `fn get_mut<Q>(self: &'_ mut Self, key: &'_ Q) -> Option` - Returns a mutable reference to the value corresponding to the key.
- `fn get_key_value<Q>(self: &'_ Self, key: &'_ Q) -> Option` - Returns the key-value pair matching the given key.
- `fn insert(self: &'_ mut Self, k: String, v: Value) -> Option` - Inserts a key-value pair into the map.
- `fn remove<Q>(self: &'_ mut Self, key: &'_ Q) -> Option` - Removes a key from the map, returning the value at the key if the key
- `fn remove_entry<Q>(self: &'_ mut Self, key: &'_ Q) -> Option` - Removes a key from the map, returning the stored key and value if the
- `fn append(self: &'_ mut Self, other: &'_ mut Self)` - Moves all elements from other into self, leaving other empty.
- `fn entry<S>(self: &'_ mut Self, key: S) -> Entry` - Gets the given key's corresponding entry in the map for in-place
- `fn len(self: &'_ Self) -> usize` - Returns the number of elements in the map.
- `fn is_empty(self: &'_ Self) -> bool` - Returns true if the map contains no elements.
- `fn iter(self: &'_ Self) -> Iter` - Gets an iterator over the entries of the map.
- `fn iter_mut(self: &'_ mut Self) -> IterMut` - Gets a mutable iterator over the entries of the map.
- `fn keys(self: &'_ Self) -> Keys` - Gets an iterator over the keys of the map.
- `fn values(self: &'_ Self) -> Values` - Gets an iterator over the values of the map.
- `fn values_mut(self: &'_ mut Self) -> ValuesMut` - Gets an iterator over mutable values of the map.
- `fn into_values(self: Self) -> IntoValues` - Gets an iterator over the values of the map.
- `fn retain<F>(self: &'_ mut Self, f: F)` - Retains only the elements specified by the predicate.
- `fn sort_keys(self: &'_ mut Self)` - Sorts this map's entries in-place using `str`'s usual ordering.

**Trait Implementations:**

- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter` - 
- **Hash**
  - `fn hash<H>(self: &'_ Self, state: &'_ mut H)` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Self) -> bool` - 
- **IntoDeserializer**
  - `fn into_deserializer(self: Self) -> <Self as >::Deserializer` - 
- **FromStr**
  - `fn from_str(s: &'_ str) -> Result` - 
- **Index**
  - `fn index(self: &'_ Self, index: &'_ Q) -> &'_ Value` - 
- **Deserializer**
  - `fn deserialize_any<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_enum<V>(self: Self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result` - 
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self` - 
- **Debug**
  - `fn fmt(self: &'_ Self, formatter: &'_ mut fmt::Formatter) -> Result` - 
- **IndexMut**
  - `fn index_mut(self: &'_ mut Self, index: &'_ Q) -> &'_ mut Value` - 
- **Extend**
  - `fn extend<T>(self: &'_ mut Self, iter: T)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Self` - 
  - `fn clone_from(self: &'_ mut Self, source: &'_ Self)` - 
- **Default**
  - `fn default() -> Self` - 
- **Serialize**
  - `fn serialize<S>(self: &'_ Self, serializer: S) -> Result` - 
- **Eq**
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result` - 



## serde_json::map::OccupiedEntry

**Type:** Struct

An occupied Entry. It is part of the [`Entry`] enum.

**Generic Parameters:**
- ''a

**Methods:**

- `fn key(self: &'_ Self) -> &'_ String` - Gets a reference to the key in the entry.
- `fn get(self: &'_ Self) -> &'_ Value` - Gets a reference to the value in the entry.
- `fn get_mut(self: &'_ mut Self) -> &'_ mut Value` - Gets a mutable reference to the value in the entry.
- `fn into_mut(self: Self) -> &'a mut Value` - Converts the entry into a mutable reference to its value.
- `fn insert(self: &'_ mut Self, value: Value) -> Value` - Sets the value of the entry with the `OccupiedEntry`'s key, and returns
- `fn remove(self: Self) -> Value` - Takes the value of the entry out of the map, and returns it.
- `fn remove_entry(self: Self) -> (String, Value)` - Removes the entry from the map, returning the stored key and value.



## serde_json::map::VacantEntry

**Type:** Struct

A vacant Entry. It is part of the [`Entry`] enum.

**Generic Parameters:**
- ''a

**Methods:**

- `fn key(self: &'_ Self) -> &'_ String` - Gets a reference to the key that would be used when inserting a value
- `fn insert(self: Self, value: Value) -> &'a mut Value` - Sets the value of the entry with the VacantEntry's key, and returns a



## serde_json::map::Values

**Type:** Struct

An iterator over a serde_json::Map's values.

**Generic Parameters:**
- ''a

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: &'_ mut Self) -> Option` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Values` - 
- **FusedIterator**
- **ExactSizeIterator**
  - `fn len(self: &'_ Self) -> usize` - 
- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 
  - `fn size_hint(self: &'_ Self) -> (usize, Option)` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 



## serde_json::map::ValuesMut

**Type:** Struct

A mutable iterator over a serde_json::Map's values.

**Generic Parameters:**
- ''a

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: &'_ mut Self) -> Option` - 
- **FusedIterator**
- **ExactSizeIterator**
  - `fn len(self: &'_ Self) -> usize` - 
- **Iterator**
  - `fn next(self: &'_ mut Self) -> Option` - 
  - `fn size_hint(self: &'_ Self) -> (usize, Option)` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 



---

# Module: `serde_json::number`

## serde_json::number::Number

**Type:** Struct

Represents a JSON number, whether integer or floating point.

**Methods:**

- `fn is_i64(self: &'_ Self) -> bool` - Returns true if the `Number` is an integer between `i64::MIN` and
- `fn is_u64(self: &'_ Self) -> bool` - Returns true if the `Number` is an integer between zero and `u64::MAX`.
- `fn is_f64(self: &'_ Self) -> bool` - Returns true if the `Number` can be represented by f64.
- `fn as_i64(self: &'_ Self) -> Option` - If the `Number` is an integer, represent it as i64 if possible. Returns
- `fn as_u64(self: &'_ Self) -> Option` - If the `Number` is an integer, represent it as u64 if possible. Returns
- `fn as_f64(self: &'_ Self) -> Option` - Represents the number as f64 if possible. Returns None otherwise.
- `fn from_f64(f: f64) -> Option` - Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON
- `fn as_i128(self: &'_ Self) -> Option` - If the `Number` is an integer, represent it as i128 if possible. Returns
- `fn as_u128(self: &'_ Self) -> Option` - If the `Number` is an integer, represent it as u128 if possible. Returns
- `fn from_i128(i: i128) -> Option` - Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or
- `fn from_u128(i: u128) -> Option` - Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only

**Trait Implementations:**

- **Eq**
- **Clone**
  - `fn clone(self: &'_ Self) -> Number` - 
- **From**
  - `fn from(i: isize) -> Self` - 
- **From**
  - `fn from(i: i32) -> Self` - 
- **From**
  - `fn from(i: i8) -> Self` - 
- **From**
  - `fn from(u: u64) -> Self` - 
- **From**
  - `fn from(u: u16) -> Self` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Number) -> bool` - 
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, formatter: &'_ mut fmt::Formatter) -> fmt::Result` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **FromStr**
  - `fn from_str(s: &'_ str) -> result::Result` - 
- **From**
  - `fn from(i: i64) -> Self` - 
- **From**
  - `fn from(i: i16) -> Self` - 
- **From**
  - `fn from(u: usize) -> Self` - 
- **From**
  - `fn from(u: u32) -> Self` - 
- **From**
  - `fn from(u: u8) -> Self` - 
- **StructuralPartialEq**
- **Deserializer**
  - `fn deserialize_any<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i8<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i16<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i32<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i64<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i128<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u8<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u16<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u32<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u64<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u128<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_f32<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_f64<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result` - 
- **Serialize**
  - `fn serialize<S>(self: &'_ Self, serializer: S) -> Result` - 
- **Display**
  - `fn fmt(self: &'_ Self, formatter: &'_ mut fmt::Formatter) -> fmt::Result` - 



---

# Module: `serde_json::read`

## serde_json::read::Fused

**Type:** Trait

Marker for whether StreamDeserializer can implement FusedIterator.



## serde_json::read::IoRead

**Type:** Struct

JSON input source that reads from a std::io input stream.

**Generic Parameters:**
- R

**Methods:**

- `fn new(reader: R) -> Self` - Create a JSON input source to read from a std::io input stream.

**Trait Implementations:**

- **Read**



## serde_json::read::Position

**Type:** Struct

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `line` | `usize` |  |
| `column` | `usize` |  |



## serde_json::read::Read

**Type:** Trait

Trait used by the deserializer for iterating over input. This is manually
"specialized" for iterating over `&[u8]`. Once feature(specialization) is
stable we can use actual specialization.

This trait is sealed and cannot be implemented for types outside of
`serde_json`.



## serde_json::read::Reference

**Type:** Enum

**Generic Parameters:**
- ''b
- ''c
- T

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Borrowed` | Tuple(&'b T) |  |
| `Copied` | Tuple(&'c T) |  |



## serde_json::read::SliceRead

**Type:** Struct

JSON input source that reads from a slice of bytes.

**Generic Parameters:**
- ''a

**Methods:**

- `fn new(slice: &'a [u8]) -> Self` - Create a JSON input source to read from a slice of bytes.

**Trait Implementations:**

- **Read**



## serde_json::read::StrRead

**Type:** Struct

JSON input source that reads from a UTF-8 string.

**Generic Parameters:**
- ''a

**Methods:**

- `fn new(s: &'a str) -> Self` - Create a JSON input source to read from a UTF-8 string.

**Trait Implementations:**

- **Read**



---

# Module: `serde_json::read::private`

## serde_json::read::private::Sealed

**Type:** Trait



---

# Module: `serde_json::ser`

## serde_json::ser::CharEscape

**Type:** Enum

Represents a character escape code in a type-safe manner.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Quote` | Unit | An escaped quote `"` |
| `ReverseSolidus` | Unit | An escaped reverse solidus `\` |
| `Solidus` | Unit | An escaped solidus `/` |
| `Backspace` | Unit | An escaped backspace character (usually escaped as `\b`) |
| `FormFeed` | Unit | An escaped form feed character (usually escaped as `\f`) |
| `LineFeed` | Unit | An escaped line feed character (usually escaped as `\n`) |
| `CarriageReturn` | Unit | An escaped carriage return character (usually escaped as `\r`) |
| `Tab` | Unit | An escaped tab character (usually escaped as `\t`) |
| `AsciiControl` | Tuple(u8) | An escaped ASCII plane control character (usually escaped as |



## serde_json::ser::CompactFormatter

**Type:** Struct

This structure compacts a JSON value with no extra whitespace.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **Default**
  - `fn default() -> CompactFormatter` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> CompactFormatter` - 
- **Formatter**



## serde_json::ser::Formatter

**Type:** Trait

This trait abstracts away serializing the JSON control characters, which allows the user to
optionally pretty print the JSON output.

**Methods:**

- `write_null`: Writes a `null` value to the specified writer.
- `write_bool`: Writes a `true` or `false` value to the specified writer.
- `write_i8`: Writes an integer value like `-123` to the specified writer.
- `write_i16`: Writes an integer value like `-123` to the specified writer.
- `write_i32`: Writes an integer value like `-123` to the specified writer.
- `write_i64`: Writes an integer value like `-123` to the specified writer.
- `write_i128`: Writes an integer value like `-123` to the specified writer.
- `write_u8`: Writes an integer value like `123` to the specified writer.
- `write_u16`: Writes an integer value like `123` to the specified writer.
- `write_u32`: Writes an integer value like `123` to the specified writer.
- `write_u64`: Writes an integer value like `123` to the specified writer.
- `write_u128`: Writes an integer value like `123` to the specified writer.
- `write_f32`: Writes a floating point value like `-31.26e+12` to the specified writer.
- `write_f64`: Writes a floating point value like `-31.26e+12` to the specified writer.
- `write_number_str`: Writes a number that has already been rendered to a string.
- `begin_string`: Called before each series of `write_string_fragment` and
- `end_string`: Called after each series of `write_string_fragment` and
- `write_string_fragment`: Writes a string fragment that doesn't need any escaping to the
- `write_char_escape`: Writes a character escape code to the specified writer.
- `write_byte_array`: Writes the representation of a byte array. Formatters can choose whether
- `begin_array`: Called before every array.  Writes a `[` to the specified
- `end_array`: Called after every array.  Writes a `]` to the specified
- `begin_array_value`: Called before every array value.  Writes a `,` if needed to
- `end_array_value`: Called after every array value.
- `begin_object`: Called before every object.  Writes a `{` to the specified
- `end_object`: Called after every object.  Writes a `}` to the specified
- `begin_object_key`: Called before every object key.
- `end_object_key`: Called after every object key.  A `:` should be written to the
- `begin_object_value`: Called before every object value.  A `:` should be written to
- `end_object_value`: Called after every object value.
- `write_raw_fragment`: Writes a raw JSON fragment that doesn't need any escaping to the



## serde_json::ser::PrettyFormatter

**Type:** Struct

This structure pretty prints a JSON value to make it human readable.

**Generic Parameters:**
- ''a

**Methods:**

- `fn new() -> Self` - Construct a pretty printer formatter that defaults to using two spaces for indentation.
- `fn with_indent(indent: &'a [u8]) -> Self` - Construct a pretty printer formatter that uses the `indent` string for indentation.

**Trait Implementations:**

- **Formatter**
  - `fn begin_array<W>(self: &'_ mut Self, writer: &'_ mut W) -> io::Result` - 
  - `fn end_array<W>(self: &'_ mut Self, writer: &'_ mut W) -> io::Result` - 
  - `fn begin_array_value<W>(self: &'_ mut Self, writer: &'_ mut W, first: bool) -> io::Result` - 
  - `fn end_array_value<W>(self: &'_ mut Self, _writer: &'_ mut W) -> io::Result` - 
  - `fn begin_object<W>(self: &'_ mut Self, writer: &'_ mut W) -> io::Result` - 
  - `fn end_object<W>(self: &'_ mut Self, writer: &'_ mut W) -> io::Result` - 
  - `fn begin_object_key<W>(self: &'_ mut Self, writer: &'_ mut W, first: bool) -> io::Result` - 
  - `fn begin_object_value<W>(self: &'_ mut Self, writer: &'_ mut W) -> io::Result` - 
  - `fn end_object_value<W>(self: &'_ mut Self, _writer: &'_ mut W) -> io::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **Default**
  - `fn default() -> Self` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> PrettyFormatter` - 



## serde_json::ser::Serializer

**Type:** Struct

A structure for serializing Rust values into JSON.

**Generic Parameters:**
- W
- F

**Methods:**

- `fn new(writer: W) -> Self` - Creates a new JSON serializer.
- `fn with_formatter(writer: W, formatter: F) -> Self` - Creates a new JSON visitor whose output will be written to the writer
- `fn into_inner(self: Self) -> W` - Unwrap the `Writer` from the `Serializer`.
- `fn pretty(writer: W) -> Self` - Creates a new JSON pretty print serializer.



## serde_json::ser::to_string

**Type:** Function

Serialize the given data structure as a String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_string<T>(value: &'_ T) -> crate::error::Result
```



## serde_json::ser::to_string_pretty

**Type:** Function

Serialize the given data structure as a pretty-printed String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_string_pretty<T>(value: &'_ T) -> crate::error::Result
```



## serde_json::ser::to_vec

**Type:** Function

Serialize the given data structure as a JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_vec<T>(value: &'_ T) -> crate::error::Result
```



## serde_json::ser::to_vec_pretty

**Type:** Function

Serialize the given data structure as a pretty-printed JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_vec_pretty<T>(value: &'_ T) -> crate::error::Result
```



## serde_json::ser::to_writer

**Type:** Function

Serialize the given data structure as JSON into the I/O stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_writer<W, T>(writer: W, value: &'_ T) -> crate::error::Result
```



## serde_json::ser::to_writer_pretty

**Type:** Function

Serialize the given data structure as pretty-printed JSON into the I/O
stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_writer_pretty<W, T>(writer: W, value: &'_ T) -> crate::error::Result
```



---

# Module: `serde_json::value`

## serde_json::value::Value

**Type:** Enum

Represents any valid JSON value.

See the [`serde_json::value` module documentation](self) for usage examples.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Null` | Unit | Represents a JSON null value. |
| `Bool` | Tuple(bool) | Represents a JSON boolean. |
| `Number` | Tuple(Number) | Represents a JSON number, whether integer or floating point. |
| `String` | Tuple(alloc::string::String) | Represents a JSON string. |
| `Array` | Tuple(alloc::vec::Vec) | Represents a JSON array. |
| `Object` | Tuple(Map) | Represents a JSON object. |

**Methods:**

- `fn get<I>(self: &'_ Self, index: I) -> Option` - Index into a JSON array or map. A string index can be used to access a
- `fn get_mut<I>(self: &'_ mut Self, index: I) -> Option` - Mutably index into a JSON array or map. A string index can be used to
- `fn is_object(self: &'_ Self) -> bool` - Returns true if the `Value` is an Object. Returns false otherwise.
- `fn as_object(self: &'_ Self) -> Option` - If the `Value` is an Object, returns the associated Map. Returns None
- `fn as_object_mut(self: &'_ mut Self) -> Option` - If the `Value` is an Object, returns the associated mutable Map.
- `fn is_array(self: &'_ Self) -> bool` - Returns true if the `Value` is an Array. Returns false otherwise.
- `fn as_array(self: &'_ Self) -> Option` - If the `Value` is an Array, returns the associated vector. Returns None
- `fn as_array_mut(self: &'_ mut Self) -> Option` - If the `Value` is an Array, returns the associated mutable vector.
- `fn is_string(self: &'_ Self) -> bool` - Returns true if the `Value` is a String. Returns false otherwise.
- `fn as_str(self: &'_ Self) -> Option` - If the `Value` is a String, returns the associated str. Returns None
- `fn is_number(self: &'_ Self) -> bool` - Returns true if the `Value` is a Number. Returns false otherwise.
- `fn as_number(self: &'_ Self) -> Option` - If the `Value` is a Number, returns the associated [`Number`]. Returns
- `fn is_i64(self: &'_ Self) -> bool` - Returns true if the `Value` is an integer between `i64::MIN` and
- `fn is_u64(self: &'_ Self) -> bool` - Returns true if the `Value` is an integer between zero and `u64::MAX`.
- `fn is_f64(self: &'_ Self) -> bool` - Returns true if the `Value` is a number that can be represented by f64.
- `fn as_i64(self: &'_ Self) -> Option` - If the `Value` is an integer, represent it as i64 if possible. Returns
- `fn as_u64(self: &'_ Self) -> Option` - If the `Value` is an integer, represent it as u64 if possible. Returns
- `fn as_f64(self: &'_ Self) -> Option` - If the `Value` is a number, represent it as f64 if possible. Returns
- `fn is_boolean(self: &'_ Self) -> bool` - Returns true if the `Value` is a Boolean. Returns false otherwise.
- `fn as_bool(self: &'_ Self) -> Option` - If the `Value` is a Boolean, returns the associated bool. Returns None
- `fn is_null(self: &'_ Self) -> bool` - Returns true if the `Value` is a Null. Returns false otherwise.
- `fn as_null(self: &'_ Self) -> Option` - If the `Value` is a Null, returns (). Returns None otherwise.
- `fn pointer(self: &'_ Self, pointer: &'_ str) -> Option` - Looks up a value by a JSON Pointer.
- `fn pointer_mut(self: &'_ mut Self, pointer: &'_ str) -> Option` - Looks up a value by a JSON Pointer and returns a mutable reference to
- `fn take(self: &'_ mut Self) -> Value` - Takes the value out of the `Value`, leaving a `Null` in its place.
- `fn sort_all_objects(self: &'_ mut Self)` - Reorders the entries of all `Value::Object` nested within this JSON

**Trait Implementations:**

- **From**
  - `fn from(f: f32) -> Self` - Convert 32-bit floating point number to `Value::Number`, or
- **Serialize**
  - `fn serialize<S>(self: &'_ Self, serializer: S) -> result::Result` - 
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self` - Create a `Value::Object` by collecting an iterator of key-value pairs.
- **From**
  - `fn from(n: u64) -> Self` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ i8) -> bool` - 
- **From**
  - `fn from(f: &'_ [T]) -> Self` - Convert a slice to `Value::Array`.
- **From**
  - `fn from(n: u16) -> Self` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ String) -> bool` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ bool) -> bool` - 
- **From**
  - `fn from(f: Vec) -> Self` - Convert a `Vec` to `Value::Array`.
- **From**
  - `fn from(n: isize) -> Self` - 
- **From**
  - `fn from(f: Cow) -> Self` - Convert copy-on-write string to `Value::String`.
- **From**
  - `fn from(n: i32) -> Self` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ str) -> bool` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ f64) -> bool` - 
- **From**
  - `fn from(n: i8) -> Self` - 
- **Index**
  - `fn index(self: &'_ Self, index: I) -> &'_ Value` - Index into a `serde_json::Value` using the syntax `value[0]` or
- **Display**
  - `fn fmt(self: &'_ Self, f: &'_ mut fmt::Formatter) -> fmt::Result` - Display a JSON value as a string.
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ f32) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **From**
  - `fn from(f: Map) -> Self` - Convert map (with string keys) to `Value::Object`.
- **StructuralPartialEq**
- **Deserializer**
  - `fn deserialize_any<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i8<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i16<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i32<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i64<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_i128<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u8<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u16<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u32<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u64<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_u128<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_f32<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_f64<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_option<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result` - 
  - `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result` - 
  - `fn deserialize_bool<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_char<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_str<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_string<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_unit<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_unit_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result` - 
  - `fn deserialize_seq<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result` - 
  - `fn deserialize_tuple_struct<V>(self: Self, _name: &'static str, _len: usize, visitor: V) -> Result` - 
  - `fn deserialize_map<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_struct<V>(self: Self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result` - 
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result` - 
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ usize) -> bool` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ u64) -> bool` - 
- **From**
  - `fn from(f: String) -> Self` - Convert `String` to `Value::String`.
- **From**
  - `fn from(f: f64) -> Self` - Convert 64-bit floating point number to `Value::Number`, or
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ u32) -> bool` - 
- **From**
  - `fn from((): ()) -> Self` - Convert `()` to `Value::Null`.
- **From**
  - `fn from(n: usize) -> Self` - 
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self` - Create a `Value::Array` by collecting an iterator of array elements.
- **Eq**
- **From**
  - `fn from(n: u32) -> Self` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ u16) -> bool` - 
- **From**
  - `fn from(array: [T; N]) -> Self` - 
- **From**
  - `fn from(n: u8) -> Self` - 
- **From**
  - `fn from(f: Number) -> Self` - Convert `Number` to `Value::Number`.
- **From**
  - `fn from(n: i64) -> Self` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ &'_ str) -> bool` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ u8) -> bool` - 
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result` - 
- **IntoDeserializer**
  - `fn into_deserializer(self: Self) -> <Self as >::Deserializer` - 
- **From**
  - `fn from(n: i16) -> Self` - 
- **IndexMut**
  - `fn index_mut(self: &'_ mut Self, index: I) -> &'_ mut Value` - Write into a `serde_json::Value` using the syntax `value[0] = ...` or
- **Default**
  - `fn default() -> Value` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ isize) -> bool` - 
- **Debug**
  - `fn fmt(self: &'_ Self, formatter: &'_ mut fmt::Formatter) -> fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Value) -> bool` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ i64) -> bool` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Value` - 
- **FromStr**
  - `fn from_str(s: &'_ str) -> Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ i32) -> bool` - 
- **From**
  - `fn from(f: &'_ str) -> Self` - Convert string slice to `Value::String`.
- **From**
  - `fn from(f: bool) -> Self` - Convert boolean to `Value::Bool`.
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ i16) -> bool` - 
- **From**
  - `fn from(opt: Option) -> Self` - 



## serde_json::value::from_value

**Type:** Function

Interpret a `serde_json::Value` as an instance of type `T`.

# Example

```
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `serde_json::Value`
    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let u: User = serde_json::from_value(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the Value does not match the
structure expected by `T`, for example if `T` is a struct type but the Value
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_value<T>(value: Value) -> Result
```



## serde_json::value::to_value

**Type:** Function

Convert a `T` into `serde_json::Value` which is an enum that can represent
any valid JSON data.

# Example

```
use serde::Serialize;
use serde_json::json;
use std::error::Error;

#[derive(Serialize)]
struct User {
    fingerprint: String,
    location: String,
}

fn compare_json_values() -> Result<(), Box<dyn Error>> {
    let u = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };

    // The type of `expected` is `serde_json::Value`
    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    });

    let v = serde_json::to_value(u).unwrap();
    assert_eq!(v, expected);

    Ok(())
}
#
# compare_json_values().unwrap();
```

# Errors

This conversion can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```
use std::collections::BTreeMap;

fn main() {
    // The keys in this map are vectors, not strings.
    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");

    println!("{}", serde_json::to_value(map).unwrap_err());
}
```

```rust
fn to_value<T>(value: T) -> Result
```



---

# Module: `serde_json::value::index`

## serde_json::value::index::Index

**Type:** Trait

A type that can be used to index into a `serde_json::Value`.

The [`get`] and [`get_mut`] methods of `Value` accept any type that
implements `Index`, as does the [square-bracket indexing operator]. This
trait is implemented for strings which are used as the index into a JSON
map, and for `usize` which is used as the index into a JSON array.

[`get`]: Value::get
[`get_mut`]: Value::get_mut
[square-bracket indexing operator]: Value#impl-Index%3CI%3E-for-Value

This trait is sealed and cannot be implemented for types outside of
`serde_json`.

# Examples

```
# use serde_json::json;
#
let data = json!({ "inner": [1, 2, 3] });

// Data is a JSON map so it can be indexed with a string.
let inner = &data["inner"];

// Inner is a JSON array so it can be indexed with an integer.
let first = &inner[0];

assert_eq!(first, 1);
```



---

# Module: `serde_json::value::index::private`

## serde_json::value::index::private::Sealed

**Type:** Trait



---

# Module: `serde_json::value::ser`

## serde_json::value::ser::SerializeMap

**Type:** Enum

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Map` | Struct (2 fields) |  |



## serde_json::value::ser::SerializeStructVariant

**Type:** Struct



## serde_json::value::ser::SerializeTupleVariant

**Type:** Struct



## serde_json::value::ser::SerializeVec

**Type:** Struct



## serde_json::value::ser::Serializer

**Type:** Struct

Serializer whose output is a `Value`.

This is the serializer that backs [`serde_json::to_value`][crate::to_value].
Unlike the main serde_json serializer which goes from some serializable
value of type `T` to JSON text, this one goes from `T` to
`serde_json::Value`.

The `to_value` function is implementable as:

```
use serde::Serialize;
use serde_json::{Error, Value};

pub fn to_value<T>(input: T) -> Result<Value, Error>
where
    T: Serialize,
{
    input.serialize(serde_json::value::Serializer)
}
```

**Unit Struct**

**Trait Implementations:**

- **Serializer**
  - `fn serialize_bool(self: Self, value: bool) -> Result` - 
  - `fn serialize_i8(self: Self, value: i8) -> Result` - 
  - `fn serialize_i16(self: Self, value: i16) -> Result` - 
  - `fn serialize_i32(self: Self, value: i32) -> Result` - 
  - `fn serialize_i64(self: Self, value: i64) -> Result` - 
  - `fn serialize_i128(self: Self, value: i128) -> Result` - 
  - `fn serialize_u8(self: Self, value: u8) -> Result` - 
  - `fn serialize_u16(self: Self, value: u16) -> Result` - 
  - `fn serialize_u32(self: Self, value: u32) -> Result` - 
  - `fn serialize_u64(self: Self, value: u64) -> Result` - 
  - `fn serialize_u128(self: Self, value: u128) -> Result` - 
  - `fn serialize_f32(self: Self, float: f32) -> Result` - 
  - `fn serialize_f64(self: Self, float: f64) -> Result` - 
  - `fn serialize_char(self: Self, value: char) -> Result` - 
  - `fn serialize_str(self: Self, value: &'_ str) -> Result` - 
  - `fn serialize_bytes(self: Self, value: &'_ [u8]) -> Result` - 
  - `fn serialize_unit(self: Self) -> Result` - 
  - `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result` - 
  - `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result` - 
  - `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &'_ T) -> Result` - 
  - `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &'_ T) -> Result` - 
  - `fn serialize_none(self: Self) -> Result` - 
  - `fn serialize_some<T>(self: Self, value: &'_ T) -> Result` - 
  - `fn serialize_seq(self: Self, len: Option) -> Result` - 
  - `fn serialize_tuple(self: Self, len: usize) -> Result` - 
  - `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result` - 
  - `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result` - 
  - `fn serialize_map(self: Self, len: Option) -> Result` - 
  - `fn serialize_struct(self: Self, name: &'static str, len: usize) -> Result` - 
  - `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result` - 
  - `fn collect_str<T>(self: Self, value: &'_ T) -> Result` - 



---

