use rust_alert::alert;

/// A custom error type used to convert error types from various crates.
#[alert(errors = [std::io::Error])]
pub struct TerminalError {}
