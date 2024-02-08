use std::{env::current_dir, fmt::Result, process::Command};
use anyhow::Result;

fn run_svn_process(inputs: &[&str]) -> Result<Vec<u8>>{
    if cfg!(target_os = "windows") {
        Ok(Command::new("svn")
            .arg(inputs)
            .output()?
            .stdout)
    } else {
        Ok(Command::new("/usr/bin/svn")
            .args(inputs)
            .output()?
            .stdout)
    }
}

fn main() -> Result<()>{
    let curdir = current_dir().unwrap().into_os_string().into_string().unwrap();
    let comm = &format!("status -v {}", curdir);

    let hello = run_svn_process(["status", "-v", &curdir])?;

    // let output = if cfg!(target_os = "windows") {
    //     Command::new("svn")
    //         .arg(comm)
    //         .output()
    //         .expect("failed to execute process")
    // } else {
    //     Command::new("/usr/bin/svn")
    //         .args(["status", "-v", &curdir])
    //         .output()
    //         .expect("failed to execute process")
    // };
    //
    // let hello = output.stdout;

    let mut last_zero_index: usize = 0;
    let mut last_dot_index: usize = 0;
    let mut last_slash_index: usize = 0;
    for (ind, i) in hello.iter().enumerate() {
        if *i == b'\n' {
            if last_dot_index < last_slash_index {
                continue;
            }
            println!("{}", String::from_utf8(hello[last_zero_index..ind].to_vec()).unwrap());
        } else if *i == b' ' {
            last_zero_index = ind;
        } else if *i == b'.' {
            last_dot_index = ind;
        } else if *i == b'/' {
            last_slash_index = ind;
        }
    }

    Ok(())
}
