use std::{
    ffi::OsStr, fmt, io,
    process::Command,
    error::Error,
};





/// This enum represesnts an error for this module
#[derive(Debug)]
pub enum CommandError {
    IO (io::Error, u32, &'static str),
    CmdFail,
}

/// Display for CommandError
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Self::IO (err, line, file) => {
                return write!(f, "Internal IO Error at {line} in {file}: {}", err);
            },
            CommandError::CmdFail => {
                return write!(f, "Command failed to execute.");
            },
        }
    }
}
impl Error for CommandError {}

/// Execute a command. The parent proc waits until the Sub process has finished executing
pub fn cmd<S: AsRef<OsStr>>(command: S, args: &[S]) -> Result<(), CommandError> {
    // Create the new command
    let mut binding = Command::new(command);
    let new_cmd = binding.args(args);


    // Spawn the new process
    match new_cmd.spawn() {
        Ok(mut proc) => {
            // Make this process wait
            match proc.wait() {
                Ok(status) => {
                    // Check whether the sub proc succeeded
                    if status.success() {
                        return Ok(());
                    } else {
                        return Err(CommandError::CmdFail);
                    }
                }
                Err(e) => {
                    return Err(CommandError::IO(e, line!(), file!()));
                }
            }
        },
        Err(e) => {
            return Err(CommandError::IO(e, line!(), file!()));
        },
    }
}
