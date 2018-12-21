use config::FileFormat;
use failure::{format_err, Error};
use ignore::overrides::OverrideBuilder;
use ignore::{Walk, WalkBuilder};
use log::{debug, info};
use std::env::current_dir;
use std::path::{self, Path, PathBuf};

use todo_r::TodoRBuilder;

/// Recurses down and try to find either .git or .todor as the root folder.
/// Ignore builder should be initialized relative to current_dir().
///
/// Returns an iterator that iterates over all the tracked files.
/// Measures are taken to make sure paths are returned in a nice relative path format.
pub fn build_walker(
    todor_builder: &mut TodoRBuilder,
    mut ignore_builder: OverrideBuilder,
) -> Result<Walk, Error> {
    info!("Looking for .git or .todor to use as workspace root...");

    let mut curr_dir = current_dir()?;
    // let mut ignore_builder = OverrideBuilder::new(&curr_dir);
    curr_dir.push(".todor");
    let mut relative_path = PathBuf::from(".");
    let mut walk_builder = WalkBuilder::new(&relative_path);
    let mut found_walker_root = false;

    for abs_path in curr_dir.ancestors() {
        // ignore previous directory to not get repeated equivalent paths
        let ignore_string = get_ignore_string(abs_path, &relative_path)?;
        debug!("adding {} in walker override", &ignore_string);
        ignore_builder.add(&ignore_string).unwrap();

        // check for .todor
        let todor_path = abs_path.with_file_name(".todor");
        if todor_path.exists() {
            found_walker_root = true;
            info!("Found workspace root: '{}'", todor_path.display());
            info!("Applying config file '{}'...", todor_path.display());

            // check for empty file before adding
            if todor_path.metadata().unwrap().len() > 2 {
                todor_builder.add_config_file_with_format(todor_path, FileFormat::Hjson)?;
            }
            break;
        }

        // check for .git
        let git_path = abs_path.with_file_name(".git");
        if git_path.exists() {
            found_walker_root = true;
            info!("Found workspace root: '{}'", git_path.display());
            break;
        }

        relative_path.push("..");
        walk_builder.add(&relative_path);
    }

    if !found_walker_root {
        return Err(format_err!(
            "No input files provided and no git repo or todor workspace found"
        ));
    }

    walk_builder
        .overrides(ignore_builder.build()?)
        .sort_by_file_name(std::ffi::OsStr::cmp)
        .add_custom_ignore_filename(".todorignore")
        .parents(false);

    Ok(walk_builder.build())
}

/// Gets the ignore string for ignore::overrides::OverrideBuilder to use.
/// Uses the fact that the file_name in abs_path is the previous directory.
fn get_ignore_string(abs_path: &Path, rel_path: &Path) -> Result<String, Error> {
    let ignore_path =
        rel_path
            .strip_prefix(".")
            .unwrap()
            .with_file_name(abs_path.file_name().ok_or_else(|| {
                format_err!("No input files provided and no git repo or todor workspace found")
            })?);

    let ignore_path_str = ignore_path.to_str().ok_or_else(|| {
        format_err!(
            "Path `{}` contains invalid Unicode and cannot be processed",
            ignore_path.to_string_lossy()
        )
    })?;

    let ignore_string = if path::MAIN_SEPARATOR != '/' {
        format!(
            "!{}",
            ignore_path_str.replace(&path::MAIN_SEPARATOR.to_string(), "/")
        )
    } else {
        format!("!{}", ignore_path_str)
    };

    Ok(ignore_string)
}
