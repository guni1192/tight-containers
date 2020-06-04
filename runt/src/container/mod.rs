use std::fs::{self, File};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

use anyhow::Result;
use chrono::{DateTime, Utc};
use nix::fcntl::{flock, FlockArg};
use nix::unistd::{Uid, User};
use serde_derive::{Deserialize, Serialize};

use crate::container::specs::{Spec, State, Status, OCI_VERSION};

pub mod specs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Container {
    pub id: String,
    pub spec: Spec,
    pub bundle: PathBuf,
    pub status: Status,
    pub created: Option<DateTime<Utc>>,
}

impl Container {
    // bundle: must absolute path
    pub fn new(id: &str, bundle: &PathBuf, spec: Spec) -> Self {
        Container {
            id: id.into(),
            bundle: bundle.to_path_buf(),
            spec,
            status: Status::Creating,
            created: None,
        }
    }

    pub fn create(&mut self) -> Result<()> {
        // assert_eq!(container.status, Status::Creating)
        self.save_metadata(&self)?;

        // -----
        // container creating
        // -----

        self.status = Status::Created;
        self.created = Some(Utc::now());
        self.save_metadata(&self)?;
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.remove_metadata()?;

        Ok(())
    }

    pub fn state(&self) -> Result<State> {
        let owner = User::from_uid(Uid::effective())?.expect("contaienr owner not detected: ");
        Ok(State {
            oci_version: OCI_VERSION.into(),
            id: self.id.clone(),
            status: self.status,
            pid: None,
            bundle: self.bundle.clone(),
            rootfs: PathBuf::from(&self.spec.root.path).canonicalize()?,
            owner: owner.name,
            annotation: None,
            created: self.created,
        })
    }
}

pub static DEFAULT_META_ROOT: &str = "/tmp/runt";
pub static METADATA_FILE: &str = "state.json";

pub trait MetadataManager {
    fn save_metadata(&self, container: &Container) -> Result<()>;
    fn remove_metadata(&self) -> Result<()>;
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
        let statefile = File::create(metadata_dir.join(METADATA_FILE))?;

        self.lock(&statefile)?;

        serde_json::to_writer(&statefile, &container)?;

        self.unlock(&statefile)?;
        Ok(())
    }

    fn remove_metadata(&self) -> Result<()> {
        let metadata_dir = PathBuf::from(DEFAULT_META_ROOT).join(&self.id);
        if !metadata_dir.exists() {
            fs::remove_dir_all(&metadata_dir)?;
        }
        Ok(())
    }

    fn load(container_id: &str) -> Result<Container> {
        let statefile_path = PathBuf::from(DEFAULT_META_ROOT)
            .join(&container_id)
            .join(METADATA_FILE);

        let statefile = File::open(statefile_path)?;
        let container: Container = serde_json::from_reader(statefile)?;
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

    pub fn cleanup(paths: &[&PathBuf]) -> Result<()> {
        for path in paths {
            if path.exists() {
                fs::remove_dir_all(&path)?;
            }
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
        testutil::cleanup(&[&bundle, &meta_dir]).unwrap();
    }

    #[test]
    fn container_state_should_be_successed() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir().unwrap();
        let rootfs = testutil::init_rootfs_dir(&bundle).unwrap();
        testutil::init_spec_file(&bundle, &rootfs).unwrap();
        let spec = specutil::load(&bundle).unwrap();

        let meta_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container_id);

        let mut container = Container::new(&container_id, &bundle, spec);
        let state = container.state().unwrap();
        let owner = User::from_uid(Uid::effective()).unwrap().unwrap();

        assert_eq!(state.id, container_id);
        assert_eq!(state.bundle, bundle);
        assert_eq!(state.rootfs, rootfs);
        assert!(state.pid.is_none());
        assert!(state.created.is_none());
        assert_eq!(state.owner, owner.name);

        assert!(container.create().is_ok());
        let state = container.state().unwrap();
        assert!(state.created.is_some());

        testutil::cleanup(&[&bundle, &meta_dir]).unwrap();
    }

    #[test]
    fn created_container_should_be_loaded_collectly() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir().unwrap();
        let rootfs = testutil::init_rootfs_dir(&bundle).unwrap();
        testutil::init_spec_file(&bundle, &rootfs).unwrap();
        let spec = specutil::load(&bundle).unwrap();

        let meta_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container_id);

        let mut container = Container::new(&container_id, &bundle, spec);

        assert!(container.create().is_ok());

        let loaded_container = Container::load(&container.id);
        assert!(loaded_container.is_ok());

        let loaded_container = loaded_container.unwrap();

        assert!(loaded_container.created.is_some());
        assert_eq!(loaded_container.id, container.id);
        assert_eq!(loaded_container.bundle, container.bundle);
        assert_eq!(loaded_container.status, container.status);

        testutil::cleanup(&[&bundle, &meta_dir]).unwrap();
    }

    #[test]
    fn container_deleting_should_be_successful() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir().unwrap();
        let rootfs = testutil::init_rootfs_dir(&bundle).unwrap();
        testutil::init_spec_file(&bundle, &rootfs).unwrap();
        let spec = specutil::load(&bundle).unwrap();

        let meta_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container_id);

        let mut container = Container::new(&container_id, &bundle, spec);
        assert!(container.create().is_ok());

        assert!(container.remove_metadata().is_ok());

        testutil::cleanup(&[&bundle, &meta_dir]).unwrap();
    }
}
