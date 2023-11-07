use anyhow::Result;
use std::{env, path::Path};

use log::{error, info};
use regex::Regex;
use walkdir::WalkDir;

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

#[cfg(windows)]
use std::os::windows::fs as windows_fs;

pub fn rsoft<P: AsRef<Path>>(
    source_directory: P,
    target_directory: Option<P>,
    suffix: Option<Vec<String>>,
    overwrite: bool,
) -> Result<()> {
    let pattern = if suffix.is_some() {
        let suffix_re = suffix
            .unwrap()
            .iter()
            .map(|s| regex::escape(s))
            .collect::<Vec<_>>()
            .join("|");

        format!(r".*\.({})$", suffix_re)
    } else {
        ".*".to_string()
    };

    let re = Regex::new(&pattern).unwrap();

    let mut target_dir = env::current_dir().unwrap();
    if target_directory.is_some() {
        target_dir = target_directory.unwrap().as_ref().to_path_buf();
    }

    // make target dir become absolute
    target_dir = target_dir.canonicalize()?;
    let source_dir = source_directory.as_ref().canonicalize()?;

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && re.is_match(&e.path().to_string_lossy()))
    {
        let path = entry.path();

        let link_name = target_dir.join(path.file_name().unwrap());

        if !link_name.exists() || overwrite {
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
