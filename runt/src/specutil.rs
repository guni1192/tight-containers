use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;

use crate::config::SPEC_FILE;
use crate::container::specs::Spec;

pub fn load(bundle: &PathBuf) -> Result<Spec> {
    let config_path = bundle.join(SPEC_FILE);
    let config_file = File::open(&config_path)?;

    let spec: Spec = serde_json::from_reader(&config_file)?;

    Ok(spec)
}

pub fn write(bundle: &PathBuf, spec: &Spec) -> Result<()> {
    let config_path = bundle.join(SPEC_FILE);
    let mut config_file = File::create(&config_path)?;

    serde_json::to_writer(&mut config_file, &spec)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use uuid::Uuid;

    use crate::container::testutil;
    use crate::container::DEFAULT_META_ROOT;

    #[test]
    fn load_container_file() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir(&container_id).unwrap();
        let meta_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container_id);

        assert!(write(&bundle, &Spec::default()).is_ok());
        std::thread::sleep(std::time::Duration::from_millis(100));

        assert!(load(&bundle).is_ok());
        testutil::cleanup(&bundle, &meta_dir).unwrap();
    }
}
