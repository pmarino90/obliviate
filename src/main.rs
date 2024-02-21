#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate humansize;
extern crate simple_logger;

use std::fs::{read_dir, Metadata};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use clap::{Arg, ArgAction, Command};
use humansize::{format_size, DECIMAL};
use log::Level;

const DAY: u64 = 86400;

struct CollectedFile {
    path: PathBuf,
    size: u64,
}

fn delete_empty_folders(start_path: &PathBuf, path: &PathBuf) {
    let is_folder_empty = |path: &PathBuf| {
        path.read_dir()
            .map(|mut i| i.next().is_none())
            .unwrap_or(false)
    };

    if is_folder_empty(&path) && path != start_path {
        trace!("Removing folder {:?}", path);
        std::fs::remove_dir(path).unwrap();
        trace!("Folder removed");

        let parent_folder = path.parent().unwrap().to_path_buf();
        delete_empty_folders(&start_path, &parent_folder);
    }
}

fn delete_files(start_path: PathBuf, collected_files: Vec<CollectedFile>, dry_run: bool) {
    let mut deleted_bytes = 0;

    for file in collected_files {
        trace!("Removing {:?}", file.path);

        if !dry_run {
            std::fs::remove_file(&file.path).unwrap();
            trace!("File removed");

            let parent_folder = file.path.parent().unwrap().to_path_buf();
            delete_empty_folders(&start_path, &parent_folder);
        }

        deleted_bytes += file.size;
    }

    info!(
        "Deleted {}.",
        format_size(deleted_bytes, DECIMAL)
    );
}

fn find_files_to_delete(folder_path: PathBuf, age: u64) -> Vec<CollectedFile> {
    let dir_content = read_dir(folder_path).unwrap();
    let today = SystemTime::now();
    let days_ago = (today - Duration::from_secs(age * DAY))
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let files_and_folders = dir_content.map(|result| {
        let res = result.unwrap();
        (res.path(), res.metadata().unwrap())
    });

    let (folders, files): (Vec<(PathBuf, Metadata)>, Vec<(PathBuf, Metadata)>) =
        files_and_folders.partition(|entry| entry.1.is_dir());

    let files_in_folders = folders
        .into_iter()
        .flat_map(|entry| find_files_to_delete(entry.0, age));

    let files_to_delete = files
        .into_iter()
        .filter(|entry| {
            let metadata = &entry.1;
            let created_time = metadata
                .created()
                .unwrap_or(metadata.modified().expect("The entry does not have a created or modified time"))
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();

            created_time <= days_ago
        })
        .map(|entry| CollectedFile {
            path: entry.0,
            size: entry.1.len(),
        });

    return files_to_delete.chain(files_in_folders).collect();
}

fn main() {
    let matches = Command::new("cargo")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Simple utility that removes file older than a certain amount of days.")
        .arg(
            Arg::new("age")
                .short('a')
                .long("age")
                .value_parser(value_parser!(u64))
                .value_name("AGE")
                .help("Number of days the file should be old to be removed.")
                .num_args(1)
                .default_value("30"),
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("When provided no files are deleted."),
        )
        .arg(
            Arg::new("PATH")
                .help("Path where to look for files to delete.")
                .required(true)
                .num_args(1)
                .index(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Outputs verbose logs to track which files are deleted."),
        )
        .get_matches();

    // NOTE: This will always return Some(value) if default_value has been set.
    let age: u64 = *matches.get_one("age").expect("Age option should always exist");
    let dry_run = matches.get_flag("dry-run");
    let verbose = matches.get_flag("verbose");
    let log_level = if verbose { Level::Trace } else { Level::Info };

    let start_path: &String = matches
        .get_one::<String>("PATH")
        .expect("A path is expected to start looking for files.");

    simple_logger::init_with_level(log_level).unwrap();

    info!("Reading folder: {}", start_path);

    let files_to_delete = find_files_to_delete(PathBuf::from(start_path), age);

    delete_files(PathBuf::from(start_path), files_to_delete, dry_run);
}
