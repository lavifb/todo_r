#[cfg(not(target_os = "macos"))]
use dirs::config_dir;

#[cfg(target_os = "macos")]
use dirs::home_dir;
#[cfg(target_os = "macos")]
use std::env;
#[cfg(target_os = "macos")]
use std::path::PathBuf;

use config::FileFormat;
use failure::Error;
use log::info;

use todo_r::TodoRBuilder;

pub fn load_global_config(builder: &mut TodoRBuilder) -> Result<(), Error> {
    #[cfg(target_os = "macos")]
    let config_dir_op = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| home_dir().map(|d| d.join(".config")));

    #[cfg(not(target_os = "macos"))]
    let config_dir_op = config_dir();

    if let Some(global_config) = config_dir_op.map(|d| d.join("todor/todor.conf")) {
        info!(
            "searching for global config in '{}'",
            global_config.display()
        );
        if global_config.exists() && global_config.metadata().unwrap().len() > 2 {
            info!("adding global config file...");
            builder.add_config_file_with_format(global_config, FileFormat::Hjson)?;
        }
    }

    Ok(())
}
