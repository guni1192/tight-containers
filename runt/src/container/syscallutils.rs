use anyhow::Result;

use nix::unistd::execve;

use std::ffi::{CStr, CString};

/// nix::unistd::execve wrapper
pub fn execve_(path: &str, args: &[String], env: &[String]) -> Result<()> {
    let path = CString::new(path)?;

    let args: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(arg.clone()).unwrap_or_default())
        .collect();
    let args: Vec<&CStr> = args.iter().map(|arg| arg.as_c_str()).collect();

    let env: Vec<CString> = env
        .iter()
        .map(|e| CString::new(e.clone()).unwrap_or_default())
        .collect();
    let env: Vec<&CStr> = env.iter().map(|e| e.as_c_str()).collect();

    execve(&path, &args, &env)?;
    Ok(())
}
