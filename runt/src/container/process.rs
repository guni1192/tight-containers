use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use nix::sys::stat::Mode;
use nix::unistd::{fork, mkfifo, ForkResult, Pid};

use crate::container::specs::Process;
use crate::container::syscallutils;

const START_TRIGGER_FIFO: &str = "start_trigger.fifo";

impl Process {
    pub fn wait_for_writing(&self, bundle: &PathBuf) -> Result<()> {
        let fifo_path = bundle.join(START_TRIGGER_FIFO);
        let mut file = File::create(&fifo_path)?;
        mkfifo(&fifo_path, Mode::all())?;
        file.write_all(b"bang")?;

        Ok(())
    }

    pub fn trigger_container_start(&self, bundle: &PathBuf) -> Result<()> {
        let fifo_path = bundle.join(START_TRIGGER_FIFO);
        let mut file = File::open(&fifo_path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(())
    }

    pub fn spawn(&self, bundle: &PathBuf) -> Result<Option<Pid>> {
        match fork()? {
            ForkResult::Parent { child } => Ok(Some(child)),
            ForkResult::Child => {
                self.wait_for_writing(bundle)?;
                syscallutils::execve_(&self.args[0], &self.args, &[])?;
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::container::specs::Spec;
    use crate::container::testutil;

    #[test]
    fn wait_for_writing_should_be_success() {
        let spec = Spec::default();
        let mut process = spec.process.clone().unwrap();
        process.args = vec!["/proc/self/exec".to_string()];
        let bundle = testutil::init_bundle_dir().unwrap();

        if let Some(_pid) = process.spawn(&bundle).unwrap() {
            for i in 1..5 {
                let result = process.trigger_container_start(&bundle);
                if result.is_ok() {
                    break;
                }
                if i == 5 && result.is_err() {
                    panic!("{:?}", result);
                }
            }
        }
    }
}
