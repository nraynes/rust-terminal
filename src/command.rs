use std::{
    ffi::OsStr,
    io::{self, Read, Write},
    path::PathBuf,
    process::{Child, Command as StdCmd, ExitStatus, Output, Stdio},
};

use crate::TerminalError;

pub struct Command {
    pipe_stdout: bool,
    current_dir: Option<PathBuf>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            pipe_stdout: false,
            current_dir: None,
        }
    }

    pub fn current_dir(&mut self, dir: PathBuf) -> &mut Self {
        self.current_dir = Some(dir);
        self
    }

    pub fn piped(&mut self) -> &mut Self {
        self.pipe_stdout = true;
        self
    }

    pub fn run<I, S>(&self, command: &str, args: I) -> Result<Output, TerminalError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        match self.pipe_stdout {
            true => self.capture_output(command, args),
            false => self.wait_for_output(command, args),
        }
    }

    /// Run command without piping output to parent process.
    fn wait_for_output<I, S>(&self, command: &str, args: I) -> Result<Output, TerminalError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = StdCmd::new(command);
        match &self.current_dir {
            Some(dir) => Ok(cmd.args(args).current_dir(dir).output()?),
            None => Ok(cmd.args(args).output()?),
        }
    }

    fn capture_output<I, S>(&self, command: &str, args: I) -> Result<Output, TerminalError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut child = self.spawn(command, args)?;
        let exit_status = Self::read_stdout_from(&mut child)?;
        Ok(Output {
            status: exit_status,
            stdout: vec![],
            stderr: vec![],
        })
    }

    /// Spawn a command and wait for it to finish, pipes stdout to parent process.
    fn spawn<I, S>(&self, command: &str, args: I) -> Result<Child, TerminalError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = StdCmd::new(command);

        let child = match &self.current_dir {
            Some(dir) => cmd
                .args(args)
                .current_dir(dir)
                .stdout(Stdio::piped())
                .spawn()?,
            None => cmd.args(args).stdout(Stdio::piped()).spawn()?,
        };
        Ok(child)
    }

    /// Takes a child process and reads its stdout while waiting for it to finish.
    pub fn read_stdout_from(child: &mut Child) -> Result<ExitStatus, TerminalError> {
        let mut child_stdout = child
            .stdout
            .take()
            .ok_or("There was a problem acquiring stdout from child process")?;
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = child_stdout.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            io::stdout().write_all(&buffer[..bytes_read])?;
        }
        Ok(child.wait()?)
    }
}
