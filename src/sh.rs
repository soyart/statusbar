use std::process::{Command, Stdio};
use std::{env, fs};

pub enum CmdError {
    /// Command spawned, but returned non-0 exit code
    ErrRun {
        code: Option<i32>,
        stdout: Option<Vec<u8>>,
        stderr: Option<Vec<u8>>,
    },

    /// Command failed to spawn
    ErrSpawn { error: std::io::Error },
}

/// Executes command `cmd` with arguments `args`.
/// Output is discarded (printed to console) and not used.
/// Throw an error if `cmd` fails to spawn or exit code != 0
pub fn exec(cmd: &str, args: &[&str]) -> Result<(), CmdError> {
    match Command::new(cmd).args(args).spawn() {
        Ok(mut result) => {
            match result.wait() {
                // Spawned but may still fail
                Ok(r) => match r.code() {
                    Some(0) => Ok(()),
                    Some(code) => Err(CmdError::ErrRun {
                        code: Some(code),
                        stdout: None,
                        stderr: None,
                    }),
                    None => Err(CmdError::ErrRun {
                        code: None,
                        stdout: None,
                        stderr: None,
                    }),
                },

                Err(error) => Err(CmdError::ErrSpawn { error }),
            }
        }

        // Failed to spawn
        Err(error) => Err(CmdError::ErrSpawn { error }),
    }
}

/// Executes command `cmd` with arguments `args`,
/// capturing output and returning stdout output as bytes,
/// or stderr output as lossy UTF-8 strings.
///
/// Throws an error if command fails to spawn
#[allow(unused)]
pub fn exec_with_output(cmd: &str, args: &[&str]) -> Result<Vec<u8>, CmdError> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|err| CmdError::ErrSpawn { error: err })?;

    if !output.status.success() {
        let stdout = Some(output.stdout);
        let stderr = Some(output.stderr);

        return Err(CmdError::ErrRun {
            code: output.status.code(),
            stdout,
            stderr,
        });
    }

    Ok(output.stdout)
}

/// Pipe stdout of `producer_cmd` to stdin of `consumer_cmd`,
/// and waits for `consumer_cmd` to finish.
/// Akin to:
/// ```shell
/// producer_cmd | consume_cmd
/// ```
/// The structure of both argument tuples is (cmd, &[arg1, arg2, ..])
pub fn pipe(
    producer_cmd: (&str, &[&str]),
    consumer_cmd: (&str, &[&str]),
) -> Result<(), CmdError> {
    let producer = Command::new(producer_cmd.0)
        .args(producer_cmd.1)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| {
            panic!(
                "failed to spawn producer {} {}",
                consumer_cmd.0,
                consumer_cmd.1.join(" ")
            )
        });

    // Ignore fdisk stderr - it will be inherited from ali-rs
    let consumer = Command::new(consumer_cmd.0)
        .args(consumer_cmd.1)
        .stdin(producer.stdout.unwrap())
        .spawn()
        .unwrap_or_else(|_| {
            panic!(
                "failed to spawn consumer {} {}",
                consumer_cmd.0,
                consumer_cmd.1.join(" ")
            )
        });

    match consumer.wait_with_output() {
        Ok(result) => match result.status.success() {
            false => Err(CmdError::ErrRun {
                code: result.status.code(),
                stdout: None,
                stderr: Some(result.stderr),
            }),
            _ => Ok(()),
        },

        Err(error) => Err(CmdError::ErrRun {
            code: None,
            stdout: None,
            stderr: Some(format!("{}", error).into()),
        }),
    }
}

// Executes cmd_str with `sh -c`:
/// ```shell
/// sh -c {cmd_str}
/// ```
///
/// cmd_str MUST NOT be surrounded beforehand
pub fn sh_c(cmd_str: &str) -> Result<(), CmdError> {
    exec("sh", &["-c", cmd_str])
}

pub fn in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }

    false
}

impl std::fmt::Debug for CmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrRun {
                code,
                stdout,
                stderr,
            } => {
                let code = match code {
                    Some(c) => c.to_string(),
                    None => "non-zero".to_string(),
                };

                let stdout = match stdout {
                    Some(ref bytes) => String::from_utf8(bytes.clone())
                        .unwrap_or("binary output".to_string()),
                    None => "ali-rs discarded stdout output".to_string(),
                };

                let stderr = match stderr {
                    Some(err) => String::from_utf8_lossy(err).into(),
                    None => "ali-rs discarded stderr output".to_string(),
                };

                write!(
                    f,
                    "run error\nexit code: {code}\nstdout: {stdout}\nstderr: {stderr}",
                )
            }
            Self::ErrSpawn { error } => write!(f, "spawn error: {error:?}"),
        }
    }
}

impl std::fmt::Display for CmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "self:?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file_exists<P>(path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        path.as_ref().exists()
    }

    #[ignore]
    #[test]
    fn test_shell() {
        exec("echo", &["hello, world!"])
            .expect("failed to execute `echo \"hello, world!\"` command");

        exec("echo", &["hello,", "world!"])
            .expect("failed to execute `echo \"hello,\" \" world!\"` command");

        exec("ls", &["-al", "./src"])
            .expect("failed to execute `ls -al ./src`");

        exec("sh", &["-c", "ls -al ./src"])
            .expect("failed to execute `sh -c \"ls -al ./src\"`");

        sh_c("ls -al ./src")
            .expect("failed to use sh_c to execute `ls -al ./src`");

        sh_c("touch ./boobs")
            .expect("failed to use sh_c to execute `touch boobs`");

        assert!(file_exists("./boobs"));

        sh_c("touch ./boobs && rm ./boobs")
            .expect("failed to use sh_c to execute `touch boobs && rm boobs`");

        assert!(!file_exists("./boobs"));
    }
}
