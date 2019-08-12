#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate humansize;
extern crate simple_logger;

use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use clap::{App, Arg};
use humansize::{file_size_opts as options, FileSize};
use log::Level;

const DAY: u64 = 86400;

struct CollectedFile {
    path: PathBuf,
    size: u64,
}

fn delete_files(collected_files: Vec<CollectedFile>, dry_run: bool) {
    let mut deleted_bytes = 0;

    for file in collected_files {
        trace!("Removing {:?}", file.path);
        if !dry_run {
            std::fs::remove_file(file.path).unwrap();
            trace!("File removed");
        }

        deleted_bytes += file.size;
    }

    info!(
        "Deleted {}.",
        deleted_bytes.file_size(options::DECIMAL).unwrap()
    );
}

fn find_files_to_delete(folder_path: PathBuf, age: &str) -> Vec<CollectedFile> {
    let dir_content = read_dir(folder_path).unwrap();
    let today = SystemTime::now();

    let files_and_folders = dir_content.map(|result| {
        let entry = result.unwrap();

        return entry;
    });

    let (folders, files): (Vec<DirEntry>, Vec<DirEntry>) =
        files_and_folders.partition(|entry| entry.metadata().unwrap().is_dir());

    let files_in_folders = folders
        .into_iter()
        .flat_map(|entry| find_files_to_delete(entry.path(), age));

    let files_to_delete = files
        .into_iter()
        .filter(|entry| {
            let metadata = entry.metadata().unwrap();
            let days_ago = (today - Duration::from_secs(age.parse::<u64>().unwrap_or(30) * DAY))
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let created_time = metadata
                .created()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();

            created_time <= days_ago
        })
        .map(|entry| CollectedFile {
            path: entry.path(),
            size: entry.metadata().unwrap().len(),
        });

    return files_to_delete.chain(files_in_folders).collect();
}

fn main() {
    let matches = App::new("Obliviate")
        .version(crate_version!())
        .author("Paolo Marino")
        .about("Simple utility that removes file older than a cerain amount of days.")
        .arg(
            Arg::with_name("age")
                .short("a")
                .long("age")
                .value_name("AGE")
                .help("Number of days the file should be old to be removed.")
                .default_value("30")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dry-run")
                .short("d")
                .long("dry-run")
                .help("When provided no files are deleted."),
        )
        .arg(
            Arg::with_name("PATH")
                .help("Path where to look for file to delete.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Outputs verbose logs to track which files are deleted."),
        )
        .get_matches();

    let age = matches.value_of("age").unwrap_or("30");
    let dry_run = matches.is_present("dry-run");
    let verbose = matches.is_present("verbose");
    let log_level = if verbose { Level::Trace } else { Level::Info };

    let start_path = matches
        .value_of("PATH")
        .expect("A path is expected to start looking for files.");

    simple_logger::init_with_level(log_level).unwrap();

    info!("Reading folder: {}", start_path);

    let files_to_delete = find_files_to_delete(PathBuf::from(start_path), age);

    delete_files(files_to_delete, dry_run);
}
