/// Main struct in crate_a
/// 
/// This struct demonstrates cross-crate references.
pub struct StructA {
    /// A field with a basic type
    pub name: String,
    /// A field referencing a type from crate_b
    pub data: crate_b::DataB,
}

impl StructA {
    /// Create a new StructA
    pub fn new(name: String, data: crate_b::DataB) -> Self {
        Self { name, data }
    }
    
    /// Process data using crate_b's functionality
    pub fn process(&self) -> String {
        crate_b::process_data(&self.data)
    }
}

/// A function that uses types from both crates
pub fn combine(a: StructA, value: i32) -> crate_b::ResultB {
    crate_b::create_result(a.name, value)
}

/// Re-export from crate_b for convenience
pub use crate_b::DataB;
