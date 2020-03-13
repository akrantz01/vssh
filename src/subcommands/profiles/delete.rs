use crate::config::Config;
use crate::util::fail;

pub fn delete(name: String, mut config: Config) {
    // Delete if exists
    config.profiles.remove(&name);

    // Commit changes
    match config.write() {
        Ok(_) => leg::success("Deleted profile if it existed", None, None),
        Err(e) => fail(&format!("Failed to write configuration file: {}", e)),
    }
}
