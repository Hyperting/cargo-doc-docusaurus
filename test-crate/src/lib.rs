/// A simple calculator library for demonstration purposes.
///
/// This library provides basic arithmetic operations and data structures
/// to showcase rustdoc-to-markdown conversion capabilities.

/// Represents a 2D point in space.
///
/// # Examples
///
/// ```
/// let point = Point { x: 10, y: 20 };
/// ```
pub struct Point {
    /// The x coordinate
    pub x: i32,
    /// The y coordinate
    pub y: i32,
}

/// A color enumeration with RGB variants.
///
/// Supports standard colors and custom RGB values.
#[derive(Debug, Clone)]
pub enum Color {
    /// Red color
    Red,
    /// Green color
    Green,
    /// Blue color
    Blue,
    /// Custom RGB color with values 0-255
    Rgb(u8, u8, u8),
}

/// Adds two numbers together.
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
///
/// The sum of `a` and `b`
///
/// # Examples
///
/// ```
/// use test_crate::add;
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
///
/// # Arguments
///
/// * `a` - First multiplicand
/// * `b` - Second multiplicand
///
/// # Returns
///
/// The product of `a` and `b`
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// A trait for shapes that can calculate their area.
pub trait Shape {
    /// Calculates the area of the shape.
    fn area(&self) -> f64;

    /// Returns the name of the shape.
    fn name(&self) -> &str;
}

/// A rectangle shape.
pub struct Rectangle {
    /// Width of the rectangle
    pub width: f64,
    /// Height of the rectangle
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

/// Maximum number of items allowed
pub const MAX_ITEMS: usize = 100;

/// Type alias for a result with our error type
pub type Result<T> = std::result::Result<T, String>;

/// A generic container for any type.
pub struct Container<T> {
    /// The value stored in the container
    pub value: T,
}

impl<T> Container<T> {
    /// Creates a new container with the given value.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}
