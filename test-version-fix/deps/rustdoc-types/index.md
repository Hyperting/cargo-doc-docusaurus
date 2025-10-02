# rustdoc_types

Rustdoc's JSON output interface

These types are the public API exposed through the `--output-format json` flag. The [`Crate`]
struct is the root of the JSON blob and all other items are contained within.

We expose a `rustc-hash` feature that is disabled by default. This feature switches the
[`std::collections::HashMap`] for [`rustc_hash::FxHashMap`] to improve the performance of said
`HashMap` in specific situations.

`cargo-semver-checks` for example, saw a [-3% improvement][1] when benchmarking using the
`aws_sdk_ec2` JSON output (~500MB of JSON). As always, we recommend measuring the impact before
turning this feature on, as [`FxHashMap`][2] only concerns itself with hash speed, and may
increase the number of collisions.

[1]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731
[2]: https://crates.io/crates/rustc-hash

## Table of Contents

- **rustdoc_types**
  - [Abi](#rustdoc_types-abi)
  - [AssocItemConstraint](#rustdoc_types-associtemconstraint)
  - [AssocItemConstraintKind](#rustdoc_types-associtemconstraintkind)
  - [Attribute](#rustdoc_types-attribute)
  - [AttributeRepr](#rustdoc_types-attributerepr)
  - [Constant](#rustdoc_types-constant)
  - [Crate](#rustdoc_types-crate)
  - [Deprecation](#rustdoc_types-deprecation)
  - [Discriminant](#rustdoc_types-discriminant)
  - [DynTrait](#rustdoc_types-dyntrait)
  - [Enum](#rustdoc_types-enum)
  - [ExternalCrate](#rustdoc_types-externalcrate)
  - [FORMAT_VERSION](#rustdoc_types-format_version)
  - [Function](#rustdoc_types-function)
  - [FunctionHeader](#rustdoc_types-functionheader)
  - [FunctionPointer](#rustdoc_types-functionpointer)
  - [FunctionSignature](#rustdoc_types-functionsignature)
  - [GenericArg](#rustdoc_types-genericarg)
  - [GenericArgs](#rustdoc_types-genericargs)
  - [GenericBound](#rustdoc_types-genericbound)
  - [GenericParamDef](#rustdoc_types-genericparamdef)
  - [GenericParamDefKind](#rustdoc_types-genericparamdefkind)
  - [Generics](#rustdoc_types-generics)
  - [Id](#rustdoc_types-id)
  - [Impl](#rustdoc_types-impl)
  - [Item](#rustdoc_types-item)
  - [ItemEnum](#rustdoc_types-itemenum)
  - [ItemKind](#rustdoc_types-itemkind)
  - [ItemSummary](#rustdoc_types-itemsummary)
  - [MacroKind](#rustdoc_types-macrokind)
  - [Module](#rustdoc_types-module)
  - [Path](#rustdoc_types-path)
  - [PolyTrait](#rustdoc_types-polytrait)
  - [PreciseCapturingArg](#rustdoc_types-precisecapturingarg)
  - [Primitive](#rustdoc_types-primitive)
  - [ProcMacro](#rustdoc_types-procmacro)
  - [ReprKind](#rustdoc_types-reprkind)
  - [Span](#rustdoc_types-span)
  - [Static](#rustdoc_types-static)
  - [Struct](#rustdoc_types-struct)
  - [StructKind](#rustdoc_types-structkind)
  - [Target](#rustdoc_types-target)
  - [TargetFeature](#rustdoc_types-targetfeature)
  - [Term](#rustdoc_types-term)
  - [Trait](#rustdoc_types-trait)
  - [TraitAlias](#rustdoc_types-traitalias)
  - [TraitBoundModifier](#rustdoc_types-traitboundmodifier)
  - [Type](#rustdoc_types-type)
  - [TypeAlias](#rustdoc_types-typealias)
  - [Union](#rustdoc_types-union)
  - [Use](#rustdoc_types-use)
  - [Variant](#rustdoc_types-variant)
  - [VariantKind](#rustdoc_types-variantkind)
  - [Visibility](#rustdoc_types-visibility)
  - [WherePredicate](#rustdoc_types-wherepredicate)


---

# Module: `rustdoc_types`

## rustdoc_types::Abi

**Type:** Enum

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Rust` | Unit | The default ABI, but that can also be written explicitly with `extern "Rust"`. |
| `C` | Struct (1 fields) | Can be specified as `extern "C"` or, as a shorthand, just `extern`. |
| `Cdecl` | Struct (1 fields) | Can be specified as `extern "cdecl"`. |
| `Stdcall` | Struct (1 fields) | Can be specified as `extern "stdcall"`. |
| `Fastcall` | Struct (1 fields) | Can be specified as `extern "fastcall"`. |
| `Aapcs` | Struct (1 fields) | Can be specified as `extern "aapcs"`. |
| `Win64` | Struct (1 fields) | Can be specified as `extern "win64"`. |
| `SysV64` | Struct (1 fields) | Can be specified as `extern "sysv64"`. |
| `System` | Struct (1 fields) | Can be specified as `extern "system"`. |
| `Other` | Tuple(String) | Any other ABI, including unstable ones. |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Abi) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Abi` - 



## rustdoc_types::AssocItemConstraint

**Type:** Struct

Describes a bound applied to an associated type/constant.

Example:
```text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
```

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name of the associated type/constant. |
| `args` | `Option` | Arguments provided to the associated type/constant. |
| `binding` | `AssocItemConstraintKind` | The kind of bound applied to the associated type/constant. |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ AssocItemConstraint) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> AssocItemConstraint` - 



## rustdoc_types::AssocItemConstraintKind

**Type:** Enum

The way in which an associate type/constant is bound.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Equality` | Tuple(Term) | The required value/type is specified exactly. e.g. |
| `Constraint` | Tuple(Vec) | The type is required to satisfy a set of bounds. |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ AssocItemConstraintKind) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> AssocItemConstraintKind` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::Attribute

**Type:** Enum

An attribute, e.g. `#[repr(C)]`

This doesn't include:
- `#[doc = "Doc Comment"]` or `/// Doc comment`. These are in [`Item::docs`] instead.
- `#[deprecated]`. These are in [`Item::deprecation`] instead.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `NonExhaustive` | Unit | `#[non_exhaustive]` |
| `MustUse` | Struct (1 fields) | `#[must_use]` |
| `MacroExport` | Unit | `#[macro_export]` |
| `ExportName` | Tuple(String) | `#[export_name = "name"]` |
| `LinkSection` | Tuple(String) | `#[link_section = "name"]` |
| `AutomaticallyDerived` | Unit | `#[automatically_derived]` |
| `Repr` | Tuple(AttributeRepr) | `#[repr]` |
| `NoMangle` | Unit | `#[no_mangle]` |
| `TargetFeature` | Struct (1 fields) | #[target_feature(enable = "feature1", enable = "feature2")] |
| `Other` | Tuple(String) | Something else. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Attribute) -> bool` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Attribute` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::AttributeRepr

**Type:** Struct

The contents of a `#[repr(...)]` attribute.

Used in [`Attribute::Repr`].

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `kind` | `ReprKind` | The representation, e.g. `#[repr(C)]`, `#[repr(transparent)]` |
| `align` | `Option` | Alignment in bytes, if explicitly specified by `#[repr(align(...)]`. |
| `packed` | `Option` | Alignment in bytes, if explicitly specified by `#[repr(packed(...)]]`. |
| `int` | `Option` | The integer type for an enum descriminant, if explicitly specified. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ AttributeRepr) -> bool` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> AttributeRepr` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::Constant

**Type:** Struct

A constant.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `expr` | `String` | The stringified expression of this constant. Note that its mapping to the original |
| `value` | `Option` | The value of the evaluated expression for this constant, which is only computed for numeric |
| `is_literal` | `bool` | Whether this constant is a bool, numeric, string, or char literal. |

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Constant` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Constant) -> bool` - 



## rustdoc_types::Crate

**Type:** Struct

The root of the emitted JSON blob.

It contains all type/documentation information
about the language items in the local crate, as well as info about external items to allow
tools to find or link to them.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `root` | `Id` | The id of the root [`Module`] item of the local crate. |
| `crate_version` | `Option` | The version string given to `--crate-version`, if any. |
| `includes_private` | `bool` | Whether or not the output includes private items. |
| `index` | `rustc_hash::FxHashMap` | A collection of all items in the local crate as well as some external traits and their |
| `paths` | `rustc_hash::FxHashMap` | Maps IDs to fully qualified paths and other info helpful for generating links. |
| `external_crates` | `rustc_hash::FxHashMap` | Maps `crate_id` of items to a crate name and html_root_url if it exists. |
| `target` | `Target` | Information about the target for which this documentation was generated |
| `format_version` | `u32` | A single version number to be used in the future when making backwards incompatible changes |

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &'_ Self) -> Crate` - 
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Eq**
- **StructuralPartialEq**
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Crate) -> bool` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 



## rustdoc_types::Deprecation

**Type:** Struct

Information about the deprecation of an [`Item`].

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `since` | `Option` | Usually a version number when this [`Item`] first became deprecated. |
| `note` | `Option` | The reason for deprecation and/or what alternatives to use. |

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Deprecation` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Deprecation) -> bool` - 



## rustdoc_types::Discriminant

**Type:** Struct

The value that distinguishes a variant in an [`Enum`] from other variants.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `expr` | `String` | The expression that produced the discriminant. |
| `value` | `String` | The numerical value of the discriminant. Stored as a string due to |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Discriminant) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Discriminant` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::DynTrait

**Type:** Struct

Dynamic trait object type (`dyn Trait`).

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `traits` | `Vec` | All the traits implemented. One of them is the vtable, and the rest must be auto traits. |
| `lifetime` | `Option` | The lifetime of the whole dyn object |

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &'_ Self) -> DynTrait` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ DynTrait) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 



## rustdoc_types::Enum

**Type:** Struct

An `enum`.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the enum. |
| `has_stripped_variants` | `bool` | Whether any variants have been removed from the result, due to being private or hidden. |
| `variants` | `Vec` | The list of variants in the enum. |
| `impls` | `Vec` | `impl`s for the enum. |

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Enum` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Enum) -> bool` - 



## rustdoc_types::ExternalCrate

**Type:** Struct

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name of the crate. |
| `html_root_url` | `Option` | The root URL at which the crate's documentation lives. |

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ ExternalCrate) -> bool` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> ExternalCrate` - 
- **StructuralPartialEq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Eq**
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 



## rustdoc_types::FORMAT_VERSION

**Type:** Constant

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as [`Crate::format_version`].
Consuming code should assert that this value matches the format version(s) that it supports.



## rustdoc_types::Function

**Type:** Struct

A function declaration (including methods and other associated functions).

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `sig` | `FunctionSignature` | Information about the function signature, or declaration. |
| `generics` | `Generics` | Information about the function’s type parameters and `where` clauses. |
| `header` | `FunctionHeader` | Information about core properties of the function, e.g. whether it's `const`, its ABI, etc. |
| `has_body` | `bool` | Whether the function has a body, i.e. an implementation. |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Function) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Function` - 



## rustdoc_types::FunctionHeader

**Type:** Struct

A set of fundamental properties of a function.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `is_const` | `bool` | Is this function marked as `const`? |
| `is_unsafe` | `bool` | Is this function unsafe? |
| `is_async` | `bool` | Is this function async? |
| `abi` | `Abi` | The ABI used by the function. |

**Trait Implementations:**

- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ FunctionHeader) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> FunctionHeader` - 
- **StructuralPartialEq**



## rustdoc_types::FunctionPointer

**Type:** Struct

A type that is a function pointer.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `sig` | `FunctionSignature` | The signature of the function. |
| `generic_params` | `Vec` | Used for Higher-Rank Trait Bounds (HRTBs) |
| `header` | `FunctionHeader` | The core properties of the function, such as the ABI it conforms to, whether it's unsafe, etc. |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ FunctionPointer) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> FunctionPointer` - 



## rustdoc_types::FunctionSignature

**Type:** Struct

The signature of a function.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `inputs` | `Vec` | List of argument names and their type. |
| `output` | `Option` | The output type, if specified. |
| `is_c_variadic` | `bool` | Whether the function accepts an arbitrary amount of trailing arguments the C way. |

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ FunctionSignature) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> FunctionSignature` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 



## rustdoc_types::GenericArg

**Type:** Enum

One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`].

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Lifetime` | Tuple(String) | A lifetime argument. |
| `Type` | Tuple(Type) | A type argument. |
| `Const` | Tuple(Constant) | A constant as a generic argument. |
| `Infer` | Unit | A generic argument that's explicitly set to be inferred. |

**Trait Implementations:**

- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ GenericArg) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> GenericArg` - 
- **StructuralPartialEq**



## rustdoc_types::GenericArgs

**Type:** Enum

A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option<u32>
                   ^^^^^
```

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `AngleBracketed` | Struct (2 fields) | `<'a, 32, B: Copy, C = u32>` |
| `Parenthesized` | Struct (2 fields) | `Fn(A, B) -> C` |
| `ReturnTypeNotation` | Unit | `T::method(..)` |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ GenericArgs) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> GenericArgs` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::GenericBound

**Type:** Enum

Either a trait bound or a lifetime bound.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `TraitBound` | Struct (3 fields) | A trait bound. |
| `Outlives` | Tuple(String) | A lifetime bound, e.g. |
| `Use` | Tuple(Vec) | `use<'a, T>` precise-capturing bound syntax |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ GenericBound) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> GenericBound` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::GenericParamDef

**Type:** Struct

One generic parameter accepted by an item.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | Name of the parameter. |
| `kind` | `GenericParamDefKind` | The kind of the parameter and data specific to a particular parameter kind, e.g. type |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ GenericParamDef) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> GenericParamDef` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::GenericParamDefKind

**Type:** Enum

The kind of a [`GenericParamDef`].

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Lifetime` | Struct (1 fields) | Denotes a lifetime parameter. |
| `Type` | Struct (3 fields) | Denotes a type parameter. |
| `Const` | Struct (2 fields) | Denotes a constant parameter. |

**Trait Implementations:**

- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ GenericParamDefKind) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> GenericParamDefKind` - 
- **StructuralPartialEq**



## rustdoc_types::Generics

**Type:** Struct

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `params` | `Vec` | A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`). |
| `where_predicates` | `Vec` | A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`). |

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Generics` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Generics) -> bool` - 



## rustdoc_types::Id

**Type:** Struct

An opaque identifier for an item.

It can be used to lookup in [`Crate::index`] or [`Crate::paths`] to resolve it
to an [`Item`].

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

**Tuple Struct** with 1 field(s)

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Copy**
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Id) -> bool` - 
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **StructuralPartialEq**
- **Eq**
- **Clone**
  - `fn clone(self: &'_ Self) -> Id` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **PartialOrd**
  - `fn partial_cmp(self: &'_ Self, other: &'_ Id) -> $crate::option::Option` - 
- **Ord**
  - `fn cmp(self: &'_ Self, other: &'_ Id) -> $crate::cmp::Ordering` - 



## rustdoc_types::Impl

**Type:** Struct

An `impl` block.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `is_unsafe` | `bool` | Whether this impl is for an unsafe trait. |
| `generics` | `Generics` | Information about the impl’s type parameters and `where` clauses. |
| `provided_trait_methods` | `Vec` | The list of the names of all the trait methods that weren't mentioned in this impl but |
| `trait_` | `Option` | The trait being implemented or `None` if the impl is inherent, which means |
| `for_` | `Type` | The type that the impl block is for. |
| `items` | `Vec` | The list of associated items contained in this impl block. |
| `is_negative` | `bool` | Whether this is a negative impl (e.g. `!Sized` or `!Send`). |
| `is_synthetic` | `bool` | Whether this is an impl that’s implied by the compiler |
| `blanket_impl` | `Option` |  |

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Impl) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Impl` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 



## rustdoc_types::Item

**Type:** Struct

Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `id` | `Id` | The unique identifier of this item. Can be used to find this item in various mappings. |
| `crate_id` | `u32` | This can be used as a key to the `external_crates` map of [`Crate`] to see which crate |
| `name` | `Option` | Some items such as impls don't have names. |
| `span` | `Option` | The source location of this item (absent if it came from a macro expansion or inline |
| `visibility` | `Visibility` | By default all documented items are public, but you can tell rustdoc to output private items |
| `docs` | `Option` | The full markdown docstring of this item. Absent if there is no documentation at all, |
| `links` | `rustc_hash::FxHashMap` | This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs |
| `attrs` | `Vec` | Attributes on this item. |
| `deprecation` | `Option` | Information about the item’s deprecation, if present. |
| `inner` | `ItemEnum` | The type-specific fields describing this item. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Item) -> bool` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Item` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::ItemEnum

**Type:** Enum

Specific fields of an item.

Part of [`Item`].

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Module` | Tuple(Module) | A module declaration, e.g. `mod foo;` or `mod foo {}` |
| `ExternCrate` | Struct (2 fields) | A crate imported via the `extern crate` syntax. |
| `Use` | Tuple(Use) | An import of 1 or more items into scope, using the `use` keyword. |
| `Union` | Tuple(Union) | A `union` declaration. |
| `Struct` | Tuple(Struct) | A `struct` declaration. |
| `StructField` | Tuple(Type) | A field of a struct. |
| `Enum` | Tuple(Enum) | An `enum` declaration. |
| `Variant` | Tuple(Variant) | A variant of a enum. |
| `Function` | Tuple(Function) | A function declaration (including methods and other associated functions) |
| `Trait` | Tuple(Trait) | A `trait` declaration. |
| `TraitAlias` | Tuple(TraitAlias) | A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;` |
| `Impl` | Tuple(Impl) | An `impl` block. |
| `TypeAlias` | Tuple(TypeAlias) | A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;` |
| `Constant` | Struct (2 fields) | The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";` |
| `Static` | Tuple(Static) | A declaration of a `static`. |
| `ExternType` | Unit | `type`s from an `extern` block. |
| `Macro` | Tuple(String) | A macro_rules! declarative macro. Contains a single string with the source |
| `ProcMacro` | Tuple(ProcMacro) | A procedural macro. |
| `Primitive` | Tuple(Primitive) | A primitive type, e.g. `u32`. |
| `AssocConst` | Struct (2 fields) | An associated constant of a trait or a type. |
| `AssocType` | Struct (3 fields) | An associated type of a trait or a type. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ ItemEnum) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> ItemEnum` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::ItemKind

**Type:** Enum

The fundamental kind of an item. Unlike [`ItemEnum`], this does not carry any additional info.

Part of [`ItemSummary`].

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Module` | Unit | A module declaration, e.g. `mod foo;` or `mod foo {}` |
| `ExternCrate` | Unit | A crate imported via the `extern crate` syntax. |
| `Use` | Unit | An import of 1 or more items into scope, using the `use` keyword. |
| `Struct` | Unit | A `struct` declaration. |
| `StructField` | Unit | A field of a struct. |
| `Union` | Unit | A `union` declaration. |
| `Enum` | Unit | An `enum` declaration. |
| `Variant` | Unit | A variant of a enum. |
| `Function` | Unit | A function declaration, e.g. `fn f() {}` |
| `TypeAlias` | Unit | A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;` |
| `Constant` | Unit | The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";` |
| `Trait` | Unit | A `trait` declaration. |
| `TraitAlias` | Unit | A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;` |
| `Impl` | Unit | An `impl` block. |
| `Static` | Unit | A `static` declaration. |
| `ExternType` | Unit | `type`s from an `extern` block. |
| `Macro` | Unit | A macro declaration. |
| `ProcAttribute` | Unit | A procedural macro attribute. |
| `ProcDerive` | Unit | A procedural macro usable in the `#[derive()]` attribute. |
| `AssocConst` | Unit | An associated constant of a trait or a type. |
| `AssocType` | Unit | An associated type of a trait or a type. |
| `Primitive` | Unit | A primitive type, e.g. `u32`. |
| `Keyword` | Unit | A keyword declaration. |
| `Attribute` | Unit | An attribute declaration. |

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &'_ Self) -> ItemKind` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ ItemKind) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Copy**
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::ItemSummary

**Type:** Struct

Information about an external (not defined in the local crate) [`Item`].

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `crate_id` | `u32` | Can be used to look up the name and html_root_url of the crate this item came from in the |
| `path` | `Vec` | The list of path components for the fully qualified path of this item (e.g. |
| `kind` | `ItemKind` | Whether this item is a struct, trait, macro, etc. |

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> ItemSummary` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ ItemSummary) -> bool` - 



## rustdoc_types::MacroKind

**Type:** Enum

The way a [`ProcMacro`] is declared to be used.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Bang` | Unit | A bang macro `foo!()`. |
| `Attr` | Unit | An attribute macro `#[foo]`. |
| `Derive` | Unit | A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]` |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> MacroKind` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ MacroKind) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Copy**



## rustdoc_types::Module

**Type:** Struct

A module declaration, e.g. `mod foo;` or `mod foo {}`.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `is_crate` | `bool` | Whether this is the root item of a crate. |
| `items` | `Vec` | [`Item`]s declared inside this module. |
| `is_stripped` | `bool` | If `true`, this module is not part of the public API, but it contains |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Module) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Module` - 



## rustdoc_types::Path

**Type:** Struct

A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` | The path of the type. |
| `id` | `Id` | The ID of the type. |
| `args` | `Option` | Generic arguments to the type. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Path) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Path` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::PolyTrait

**Type:** Struct

A trait and potential HRTBs

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `trait_` | `Path` | The path to the trait. |
| `generic_params` | `Vec` | Used for Higher-Rank Trait Bounds (HRTBs) |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ PolyTrait) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> PolyTrait` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::PreciseCapturingArg

**Type:** Enum

One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Lifetime` | Tuple(String) | A lifetime. |
| `Param` | Tuple(String) | A type or constant parameter. |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ PreciseCapturingArg) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> PreciseCapturingArg` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::Primitive

**Type:** Struct

A primitive type declaration. Declarations of this kind can only come from the core library.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name of the type. |
| `impls` | `Vec` | The implementations, inherent and of traits, on the primitive type. |

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Primitive) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Primitive` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 



## rustdoc_types::ProcMacro

**Type:** Struct

A procedural macro.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `kind` | `MacroKind` | How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]` |
| `helpers` | `Vec` | Helper attributes defined by a macro to be used inside it. |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ ProcMacro) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> ProcMacro` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::ReprKind

**Type:** Enum

The kind of `#[repr]`.

See [AttributeRepr::kind]`.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Rust` | Unit | `#[repr(Rust)]` |
| `C` | Unit | `#[repr(C)]` |
| `Transparent` | Unit | `#[repr(transparent)] |
| `Simd` | Unit | `#[repr(simd)]` |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ ReprKind) -> bool` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> ReprKind` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::Span

**Type:** Struct

A range of source code.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `filename` | `std::path::PathBuf` | The path to the source file for this span relative to the path `rustdoc` was invoked with. |
| `begin` | `(usize, usize)` | One indexed Line and Column of the first character of the `Span`. |
| `end` | `(usize, usize)` | One indexed Line and Column of the last character of the `Span`. |

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Span) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Span` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 



## rustdoc_types::Static

**Type:** Struct

A `static` declaration.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `type_` | `Type` | The type of the static. |
| `is_mutable` | `bool` | This is `true` for mutable statics, declared as `static mut X: T = f();` |
| `expr` | `String` | The stringified expression for the initial value. |
| `is_unsafe` | `bool` | Is the static `unsafe`? |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Static) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Static` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::Struct

**Type:** Struct

A `struct`.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `kind` | `StructKind` | The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it, |
| `generics` | `Generics` | The generic parameters and where clauses on this struct. |
| `impls` | `Vec` | All impls (both of traits and inherent) for this struct. |

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Struct` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Struct) -> bool` - 



## rustdoc_types::StructKind

**Type:** Enum

The kind of a [`Struct`] and the data specific to it, i.e. fields.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Unit` | Unit | A struct with no fields and no parentheses. |
| `Tuple` | Tuple(Vec) | A struct with unnamed fields. |
| `Plain` | Struct (2 fields) | A struct with named fields. |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ StructKind) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> StructKind` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::Target

**Type:** Struct

Information about a target

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `triple` | `String` | The target triple for which this documentation was generated |
| `target_features` | `Vec` | A list of features valid for use in `#[target_feature]` attributes |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Target) -> bool` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Target` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::TargetFeature

**Type:** Struct

Information about a target feature.

Rust target features are used to influence code generation, especially around selecting
instructions which are not universally supported by the target architecture.

Target features are commonly enabled by the [`#[target_feature]` attribute][1] to influence code
generation for a particular function, and less commonly enabled by compiler options like
`-Ctarget-feature` or `-Ctarget-cpu`. Targets themselves automatically enable certain target
features by default, for example because the target's ABI specification requires saving specific
registers which only exist in an architectural extension.

Target features can imply other target features: for example, x86-64 `avx2` implies `avx`, and
aarch64 `sve2` implies `sve`, since both of these architectural extensions depend on their
predecessors.

Target features can be probed at compile time by [`#[cfg(target_feature)]`][2] or `cfg!(…)`
conditional compilation to determine whether a target feature is enabled in a particular
context.

[1]: https://doc.rust-lang.org/stable/reference/attributes/codegen.html#the-target_feature-attribute
[2]: https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name of this target feature. |
| `implies_features` | `Vec` | Other target features which are implied by this target feature, if any. |
| `unstable_feature_gate` | `Option` | If this target feature is unstable, the name of the associated language feature gate. |
| `globally_enabled` | `bool` | Whether this feature is globally enabled for this compilation session. |

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ TargetFeature) -> bool` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> TargetFeature` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::Term

**Type:** Enum

Either a type or a constant, usually stored as the right-hand side of an equation in places like
[`AssocItemConstraint`]

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Type` | Tuple(Type) | A type. |
| `Constant` | Tuple(Constant) | A constant. |

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Term` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Term) -> bool` - 



## rustdoc_types::Trait

**Type:** Struct

A `trait` declaration.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `is_auto` | `bool` | Whether the trait is marked `auto` and is thus implemented automatically |
| `is_unsafe` | `bool` | Whether the trait is marked as `unsafe`. |
| `is_dyn_compatible` | `bool` | Whether the trait is [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)[^1]. |
| `items` | `Vec` | Associated [`Item`]s that can/must be implemented by the `impl` blocks. |
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the trait. |
| `bounds` | `Vec` | Constraints that must be met by the implementor of the trait. |
| `implementations` | `Vec` | The implementations of the trait. |

**Trait Implementations:**

- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Trait) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Trait` - 
- **StructuralPartialEq**



## rustdoc_types::TraitAlias

**Type:** Struct

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the alias. |
| `params` | `Vec` | The bounds that are associated with the alias. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ TraitAlias) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> TraitAlias` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::TraitBoundModifier

**Type:** Enum

A set of modifiers applied to a trait.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `None` | Unit | Marks the absence of a modifier. |
| `Maybe` | Unit | Indicates that the trait bound relaxes a trait bound applied to a parameter by default, |
| `MaybeConst` | Unit | Indicates that the trait bound must be applicable in both a run-time and a compile-time |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> TraitBoundModifier` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ TraitBoundModifier) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Copy**



## rustdoc_types::Type

**Type:** Enum

A type.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `ResolvedPath` | Tuple(Path) | Structs, enums, unions and type aliases, e.g. `std::option::Option<u32>` |
| `DynTrait` | Tuple(DynTrait) | Dynamic trait object type (`dyn Trait`). |
| `Generic` | Tuple(String) | Parameterized types. The contained string is the name of the parameter. |
| `Primitive` | Tuple(String) | Built-in numeric types (e.g. `u32`, `f32`), `bool`, `char`. |
| `FunctionPointer` | Tuple(Box) | A function pointer type, e.g. `fn(u32) -> u32`, `extern "C" fn() -> *const u8` |
| `Tuple` | Tuple(Vec) | A tuple type, e.g. `(String, u32, Box<usize>)` |
| `Slice` | Tuple(Box) | An unsized slice type, e.g. `[u32]`. |
| `Array` | Struct (2 fields) | An array type, e.g. `[u32; 15]` |
| `Pat` | Struct (1 fields) | A pattern type, e.g. `u32 is 1..` |
| `ImplTrait` | Tuple(Vec) | An opaque type that satisfies a set of bounds, `impl TraitA + TraitB + ...` |
| `Infer` | Unit | A type that's left to be inferred, `_` |
| `RawPointer` | Struct (2 fields) | A raw pointer type, e.g. `*mut u32`, `*const u8`, etc. |
| `BorrowedRef` | Struct (3 fields) | `&'a mut String`, `&str`, etc. |
| `QualifiedPath` | Struct (4 fields) | Associated types like `<Type as Trait>::Name` and `T::Item` where |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Type) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Type` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::TypeAlias

**Type:** Struct

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `type_` | `Type` | The type referred to by this alias. |
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the alias. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ TypeAlias) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> TypeAlias` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::Union

**Type:** Struct

A `union`.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `generics` | `Generics` | The generic parameters and where clauses on this union. |
| `has_stripped_fields` | `bool` | Whether any fields have been removed from the result, due to being private or hidden. |
| `fields` | `Vec` | The list of fields in the union. |
| `impls` | `Vec` | All impls (both of traits and inherent) for this union. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Union) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Union` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::Use

**Type:** Struct

A `use` statement.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `source` | `String` | The full path being imported. |
| `name` | `String` | May be different from the last segment of `source` when renaming imports: |
| `id` | `Option` | The ID of the item being imported. Will be `None` in case of re-exports of primitives: |
| `is_glob` | `bool` | Whether this statement is a wildcard `use`, e.g. `use source::*;` |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Use) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Use` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::Variant

**Type:** Struct

A variant of an enum.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `kind` | `VariantKind` | Whether the variant is plain, a tuple-like, or struct-like. Contains the fields. |
| `discriminant` | `Option` | The discriminant, if explicitly specified. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Variant) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Variant` - 
- **StructuralPartialEq**
- **Eq**



## rustdoc_types::VariantKind

**Type:** Enum

The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields.

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Plain` | Unit | A variant with no parentheses |
| `Tuple` | Tuple(Vec) | A variant with unnamed fields. |
| `Struct` | Struct (2 fields) | A variant with named fields. |

**Trait Implementations:**

- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ VariantKind) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> VariantKind` - 



## rustdoc_types::Visibility

**Type:** Enum

Visibility of an [`Item`].

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Public` | Unit | Explicitly public visibility set with `pub`. |
| `Default` | Unit | For the most part items are private by default. The exceptions are associated items of |
| `Crate` | Unit | Explicitly crate-wide visibility set with `pub(crate)` |
| `Restricted` | Struct (2 fields) | For `pub(in path)` visibility. |

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ Visibility) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> Visibility` - 
- **StructuralPartialEq**
- **Eq**
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 



## rustdoc_types::WherePredicate

**Type:** Enum

One `where` clause.
```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `BoundPredicate` | Struct (3 fields) | A type is expected to comply with a set of bounds |
| `LifetimePredicate` | Struct (2 fields) | A lifetime is expected to outlive other lifetimes. |
| `EqPredicate` | Struct (2 fields) | A type must exactly equal another type. |

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result` - 
- **Serialize**
  - `fn serialize<__S>(self: &'_ Self, __serializer: __S) -> _serde::__private228::Result` - 
- **Debug**
  - `fn fmt(self: &'_ Self, f: &'_ mut $crate::fmt::Formatter) -> $crate::fmt::Result` - 
- **PartialEq**
  - `fn eq(self: &'_ Self, other: &'_ WherePredicate) -> bool` - 
- **Hash**
  - `fn hash<__H>(self: &'_ Self, state: &'_ mut __H)` - 
- **Clone**
  - `fn clone(self: &'_ Self) -> WherePredicate` - 
- **StructuralPartialEq**
- **Eq**



---

