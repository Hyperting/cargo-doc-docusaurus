/// Data structure in crate_b
/// 
/// This is a core type used across the workspace.
#[derive(Debug, Clone)]
pub struct DataB {
    /// The data value
    pub value: i32,
    /// A label for the data
    pub label: String,
}

impl DataB {
    /// Create new DataB
    pub fn new(value: i32, label: String) -> Self {
        Self { value, label }
    }
}

/// Result type for operations
pub struct ResultB {
    /// Success status
    pub success: bool,
    /// Result message
    pub message: String,
}

/// Process data and return a description
pub fn process_data(data: &DataB) -> String {
    format!("{}: {}", data.label, data.value)
}

/// Create a result with the given parameters
pub fn create_result(name: String, code: i32) -> ResultB {
    ResultB {
        success: code >= 0,
        message: format!("{} - code: {}", name, code),
    }
}

/// A trait defined in crate_b
pub trait Processor {
    /// Process and return a string
    fn process(&self) -> String;
}

impl Processor for DataB {
    fn process(&self) -> String {
        process_data(self)
    }
}
