use std::{env, ffi::OsStr, fs::{self, ReadDir}, path::PathBuf};

use id3::{Tag, TagLike};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let scan_path = &args[1];
    println!("path: {}", scan_path);

    let paths = fs::read_dir(scan_path).unwrap_or_else(|err| {
        eprintln!("Error reading path: {}. {}", scan_path, err);
        std::process::exit(1);
    });

    process_path(paths);
}

fn process_path(dir_paths: ReadDir)
{
    for path_result in dir_paths {
        match path_result {
            Ok(path) => {
                if path.path().is_dir() {
                    match path.path().read_dir() {
                        Ok(paths) => process_path(paths),
                        Err(err) => eprintln!("Error reading path: {}", err)
                    }
                } else if path.path().is_file() {
                    process_mp3(&path.path())
                }
            },
            Err(err) => eprintln!("Error reading path: {}", err),
        }
    }
}

fn process_mp3(path: &PathBuf)
{
    if path.extension().and_then(OsStr::to_str) != Some("mp3") {
        return;
    }

    let tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        Err(err) => {
            println!("Couldn't read MP3 {} or MP3 has no id3 tags. {}", path.display(), err);
            return;
        }
    };

    if tag.artist().is_none() {
        println!("MP3 {} has no artist defined", path.display());
        return;
    }

    if tag.title().is_none() {
        println!("MP3 {} has no title defined", path.display());
    }
}
