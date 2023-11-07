use anyhow::Result;
use std::{env, path::Path};

use log::{error, info};
use regex::Regex;
use walkdir::WalkDir;

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

#[cfg(windows)]
use std::os::windows::fs as windows_fs;

pub fn rsoft<P: AsRef<Path>>(target_directory: P, suffix: Option<String>) -> Result<()> {
    let pattern = if suffix.is_some() {
        format!(r".*\.{}", suffix.unwrap())
    } else {
        ".*".to_string()
    };

    let re = Regex::new(&pattern).unwrap();
    let current_directory = env::current_dir().unwrap();

    for entry in WalkDir::new(target_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && re.is_match(&e.path().to_string_lossy()))
    {
        let path = entry.path();

        let link_name = current_directory.join(path.file_name().unwrap());

        if !link_name.exists() {
            #[cfg(unix)]
            {
                if let Err(e) = unix_fs::symlink(path, &link_name) {
                    error!("Failed to create symlink: {}", e);
                }
            }

            #[cfg(windows)]
            {
                if let Err(e) = windows_fs::symlink_file(path, &link_name) {
                    error!("Failed to create symlink: {}", e);
                }
            }
            info!("Created symlink {:?} -> {:?}", link_name, path);
        } else {
            info!("Symlink {:?} already exists.", link_name);
        }
    }

    Ok(())
}
