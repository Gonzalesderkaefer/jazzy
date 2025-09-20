use std::{
    ffi::OsStr,
    io, process::Command,
};





/// This enum represesnts an error for this module
pub enum CommandError {
    IO (io::Error),
    CmdFail,
}



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
                    return Err(CommandError::IO(e));
                }
            }
        },
        Err(e) => {
            return Err(CommandError::IO(e));
        },
    }
}
