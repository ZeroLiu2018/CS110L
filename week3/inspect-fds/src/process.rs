use std::fmt;
use std::fmt::Formatter;
use crate::open_file::OpenFile;
#[allow(unused)] // TODO: delete this line for Milestone 3
use std::fs;
use std::process::{Command, Stdio};
use crate::process;

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub pid: usize,
    pub ppid: usize,
    pub command: String,
}

impl Process {
    #[allow(unused)] // TODO: delete this line for Milestone 1
    pub fn new(pid: usize, ppid: usize, command: String) -> Process {
        Process { pid, ppid, command }
    }

    /// This function returns a list of file descriptor numbers for this Process, if that
    /// information is available (it will return None if the information is unavailable). The
    /// information will commonly be unavailable if the process has exited. (Zombie processes
    /// still have a pid, but their resources have already been freed, including the file
    /// descriptor table.)
    pub fn list_fds(&self) -> Option<Vec<usize>> {
        let mut lsof_cmd = Command::new("lsof")
            .arg("-X").arg("-p").arg(&self.pid.to_string()).
            stdout(Stdio::piped()).spawn().ok()?;
        let awk_output = Command::new("awk")
            .arg("NR>1 {print $4}").
            stdin(lsof_cmd.stdout.take().unwrap()).output().ok()?;
        let mut output: Vec<usize> = Vec::new();
        for mut _line in String::from_utf8(awk_output.stdout).ok()?.lines() {
            let mut line = _line.to_string();
            println!("line={}", line);
            match line.as_bytes().last() {
                None => {}
                Some(ch) => {if !ch.is_ascii_digit(){
                    line.pop();
                }}
            }
            output.push(line.parse().unwrap());
        }
        return Some(output);
    }

    /// This function returns a list of (fdnumber, OpenFile) tuples, if file descriptor
    /// information is available (it returns None otherwise). The information is commonly
    /// unavailable if the process has already exited.
    pub fn list_open_files(&self) -> Option<Vec<(usize, OpenFile)>> {
        let mut open_files = vec![];
        for fd in self.list_fds()? {
            open_files.push((fd, OpenFile::from_fd(self.pid, fd)?));
        }
        Some(open_files)
    }

    pub fn print(&self) {
        let begin = format!("========== {} ==========", self);
        let end = "=".repeat(begin.len());
        println!("{}", begin);
        let fds = self.list_fds().unwrap_or_default();
        println!("========== This Process has {} fd ==========", fds.len());
        for (i, fd) in fds.iter().enumerate() {
            println!("fd{} = {}", i, fd)
        }
        println!("{}", end);
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" (pid {}, ppid{})", self.command, self.pid, self.ppid)
    }
}

#[cfg(test)]
mod test {
    use crate::ps_utils;
    use std::process::{Child, Command};

    fn start_c_program(program: &str) -> Child {
        Command::new(program)
            .spawn()
            .expect(&format!("Could not find {}. Have you run make?", program))
    }

    #[test]
    fn test_list_fds() {
        let mut test_subprocess = start_c_program("./multi_pipe_test");
        let process = ps_utils::get_target("multi_pipe_test").unwrap().unwrap();
        assert_eq!(
            process
                .list_fds()
                .expect("Expected list_fds to find file descriptors, but it returned None"),
            vec![0, 1, 2, 4, 5]
        );
        let _ = test_subprocess.kill();
    }

    #[test]
    fn test_list_fds_zombie() {
        let mut test_subprocess = start_c_program("./nothing");
        let process = ps_utils::get_target("nothing").unwrap().unwrap();
        assert!(
            process.list_fds().is_none(),
            "Expected list_fds to return None for a zombie process"
        );
        let _ = test_subprocess.kill();
    }
}
