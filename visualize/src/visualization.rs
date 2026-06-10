/// A single named expectation failure to display.
///
/// `message` constructs a failing assertion and returns its failure message, so the
/// differing value type of each assertion is erased to `String` before it is stored.
pub struct Visualization {
    /// The header group this visualization belongs to, e.g. "string".
    pub header: &'static str,
    /// The expectation method name, e.g. "to_start_with".
    pub name: &'static str,
    /// Produces the failure message by constructing and checking a failing assertion.
    pub message: fn() -> String,
}
