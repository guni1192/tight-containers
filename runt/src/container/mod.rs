use std::fs::{self, File};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

use anyhow::Result;
use nix::fcntl::{flock, FlockArg};
use serde_derive::{Deserialize, Serialize};

use crate::container::specs::{Spec, State, Status, OCI_VERSION};

pub mod specs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Container {
    pub id: String,
    pub spec: Spec,
    pub bundle: PathBuf,
    pub status: Status,
}

impl Container {
    pub fn new(id: &str, bundle: &PathBuf, spec: Spec) -> Self {
        Container {
            id: id.into(),
            bundle: bundle.to_path_buf(),
            spec,
            status: Status::Creating,
        }
    }

    pub fn create(&mut self) -> Result<()> {
        // assert_eq!(container.status, Status::Creating)
        self.save_metadata(&self)?;

        // -----
        // container creating
        // -----

        self.status = Status::Created;
        self.save_metadata(&self)?;
        Ok(())
    }

    pub fn state(&self) -> Result<State> {
        Ok(State {
            oci_version: OCI_VERSION.into(),
            id: self.id.clone(),
            status: self.status,
            pid: None,
            bundle: self.bundle.clone(),
            rootfs: PathBuf::from(&self.spec.root.path),
            owner: "root".into(),
            annotation: None,
            created: None,
        })
    }
}

pub static DEFAULT_META_ROOT: &str = "/tmp/runt";

pub trait MetadataManager {
    fn save_metadata(&self, container: &Container) -> Result<()>;
    fn load(container_id: &str) -> Result<Container>;
    fn lock(&self, file: &File) -> Result<()>;
    fn unlock(&self, file: &File) -> Result<()>;
}

impl MetadataManager for Container {
    fn save_metadata(&self, container: &Container) -> Result<()> {
        let metadata_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container.id);
        if !metadata_dir.exists() {
            fs::create_dir_all(&metadata_dir)?;
        }
        let statefile = File::create(metadata_dir.join("state.json"))?;

        self.lock(&statefile)?;

        serde_json::to_writer(&statefile, &container)?;

        self.unlock(&statefile)?;
        Ok(())
    }
    fn load(container_id: &str) -> Result<Container> {
        let container = Container {
            id: "hoge".into(),
            bundle: PathBuf::from("."),
            spec: Spec::default(),
            status: Status::Created,
        };
        Ok(container)
    }

    fn lock(&self, file: &File) -> Result<()> {
        let fd = file.as_raw_fd();
        flock(fd, FlockArg::LockExclusive)?;
        Ok(())
    }

    fn unlock(&self, file: &File) -> Result<()> {
        let fd = file.as_raw_fd();
        flock(fd, FlockArg::Unlock)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod testutil {
    use super::*;
    use std::fs;

    use uuid::Uuid;

    use crate::specutil;

    pub fn init_bundle_dir() -> Result<PathBuf> {
        let bundle = tempfile::tempdir()?.into_path();
        fs::create_dir_all(&bundle)?;
        Ok(bundle)
    }

    pub fn init_rootfs_dir(bundle: &PathBuf) -> Result<PathBuf> {
        let dir_name = Uuid::new_v4().to_string();
        let rootfs = bundle.join(dir_name);
        fs::create_dir_all(&rootfs)?;
        Ok(rootfs)
    }

    pub fn init_spec_file(bundle: &PathBuf, rootfs: &PathBuf) -> Result<()> {
        let mut spec = Spec::default();
        spec.root.path = rootfs.clone().to_str().unwrap().to_string();

        specutil::write(&bundle, &spec)?;
        Ok(())
    }

    pub fn cleanup(bundle: &PathBuf, meta_dir: &PathBuf) -> Result<()> {
        // bundledir have rootfs, config.json
        if bundle.exists() {
            fs::remove_dir_all(&bundle)?;
        }

        // remove statefile(e.g /run/runt/<container-id>/state.json)
        if meta_dir.exists() {
            fs::remove_dir_all(&meta_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use uuid::Uuid;

    use crate::specutil;

    #[test]
    fn container_creating_should_be_successed() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir().unwrap();
        let rootfs = testutil::init_rootfs_dir(&bundle).unwrap();
        testutil::init_spec_file(&bundle, &rootfs).unwrap();
        let spec = specutil::load(&bundle).unwrap();

        let meta_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container_id);

        let mut container = Container::new(&container_id, &bundle, spec);
        assert_eq!(container.id, container_id);
        assert_eq!(container.bundle, bundle);
        assert_eq!(container.status, Status::Creating);

        assert!(container.create().is_ok());

        assert_eq!(container.status, Status::Created);
        testutil::cleanup(&bundle, &meta_dir).unwrap();
    }

    #[test]
    fn container_state_should_be_successed() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir().unwrap();
        let rootfs = testutil::init_rootfs_dir(&bundle).unwrap();
        testutil::init_spec_file(&bundle, &rootfs).unwrap();
        let spec = specutil::load(&bundle).unwrap();

        let meta_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container_id);

        let container = Container::new(&container_id, &bundle, spec);
        let state = container.state().unwrap();
        assert_eq!(state.id, container_id);
        assert_eq!(state.bundle, bundle);
        assert_eq!(state.rootfs, rootfs);
        assert!(state.pid.is_none());

        testutil::cleanup(&bundle, &meta_dir).unwrap();
    }
}
