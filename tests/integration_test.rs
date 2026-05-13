use gag::BufferRedirect;
use rust_terminal::Terminal;

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn test_run_command_cap() {
        let output = Terminal::command()
            .piped()
            .run("echo", ["Hello world!"])
            .unwrap();
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn test_output_capture() {
        let stdout_buf = BufferRedirect::stdout().unwrap();
        Terminal::command()
            .piped()
            .run("echo", ["Hello world!"])
            .unwrap();
        Terminal::command()
            .run("echo", ["The quick brown fox."])
            .unwrap();
        let mut output = stdout_buf.into_inner();
        let mut output_str = String::new();
        output.read_to_string(&mut output_str).unwrap();
        assert!(output_str.contains("Hello world!"));
        assert!(!output_str.contains("The quick brown fox."));
    }

    #[test]
    fn test_run_command_no_cap() {
        let output = Terminal::command().run("echo", ["Hello world!"]).unwrap();
        assert_eq!(output.status.code(), Some(0));
    }
}
