use std::path::PathBuf;

use anyhow::Result;

use crate::container::specs::{Spec, State, Status, OCI_VERSION};

pub mod specs;

pub struct Container {
    pub id: String,
    pub spec: Spec,
    pub bundle: PathBuf,
    // pub state: State,
    // pub metadata: Metadata
}

impl Container {
    pub fn new(id: &str, bundle: &PathBuf, spec: Spec) -> Self {
        Container {
            id: id.into(),
            bundle: bundle.to_path_buf(),
            spec,
        }
    }

    pub fn create(&self) -> Result<State> {
        Ok(State {
            oci_version: OCI_VERSION.into(),
            id: self.id.clone(),
            status: Status::Created,
            pid: None,
            bundle: self.bundle.clone(),
            rootfs: PathBuf::from(self.spec.root.path.clone()),
            owner: "root".into(), // TODO: getusername
            annotation: None,
            created: None, // TODO:
        })
    }
}

#[cfg(test)]
pub mod testutil {
    use super::*;
    use std::fs;

    use uuid::Uuid;

    use crate::specutil;

    pub fn init_bundle_dir(container_id: &str) -> Result<PathBuf> {
        let base = tempfile::tempdir()?.into_path();
        let bundle = base.join(container_id);
        fs::create_dir_all(&bundle)?;
        Ok(bundle)
    }

    pub fn init_rootfs_dir(bundle: &PathBuf) -> Result<PathBuf> {
        let dir_name = Uuid::new_v4().to_string();
        let rootfs = bundle.join(dir_name);
        fs::create_dir_all(&rootfs)?;
        Ok(rootfs)
    }

    pub fn init_spec_file(bundle: &PathBuf) -> Result<()> {
        let mut spec = Spec::default();
        spec.root.path = bundle.clone().to_str().unwrap().to_string();

        specutil::write(&bundle, &spec)?;
        Ok(())
    }

    pub fn cleanup(bundle: &PathBuf) -> Result<()> {
        fs::remove_dir_all(&bundle)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use uuid::Uuid;

    #[test]
    fn bundle_should_be_current_dir() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir(&container_id).unwrap();
        let rootfs = testutil::init_rootfs_dir(&bundle).unwrap();
        let mut spec = Spec::default();
        spec.root.path = rootfs.to_str().unwrap().into();

        let container = Container::new(&container_id, &bundle, spec);

        let state = container.create().unwrap();

        assert_eq!(state.oci_version, OCI_VERSION.to_string());
        assert_eq!(state.id, container_id.to_string());
        assert_eq!(state.bundle, bundle);
        assert_eq!(state.rootfs, rootfs);
        assert_eq!(state.status, Status::Created);
        testutil::cleanup(&bundle).unwrap();
    }
}
