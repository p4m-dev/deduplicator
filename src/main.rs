
use std::io::{self, Read};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use pbr::ProgressBar;

struct File {
    path: String,
    size: u64
}

fn get_files(path: &Path, recursively: bool) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();
        let path = Path::new(&entry_path);
        if recursively && path.is_dir() {
            files.append(&mut get_files(path, true)?);
        }
        if path.is_dir() {
            continue;
        }
        if let Some(path_str) = path.to_str() {
            files.push(String::from(path_str))
        }
    }

    Ok(files)
}

fn get_checksum(path: &String) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(path)?;
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    Ok(format!("{:X}", hash))
}

fn get_file_size(path: &String) -> io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

fn main() {
    let files = get_files(Path::new("."), false).expect("Error while getting files");
    let mut file_data: HashMap<String, Vec<File>> = HashMap::new();
    let mut progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.format("<--.>");

    for file in files {
        let checksum = get_checksum(&file).expect("Error while getting checksum");
        let size = get_file_size(&file).expect("Error while getting size");
        let data = File {
            path: file,
            size
        };

        if file_data.contains_key(&checksum) {
            if let Some(file_list) = file_data.get_mut(&checksum) {
                file_list.push(data);
            }
        } else {
            file_data.insert(checksum, vec![data]);
        }
        progress_bar.inc();
    }

    
    let mut delete_candidates = Vec::new();
    for (_, file_list) in file_data.iter_mut() {
        if file_list.len() > 1 {
            for file in file_list[1..].iter_mut() {
                delete_candidates.push(&file.path);
                println!("{}", file.path, );
            }
        }
    }

    for path in delete_candidates {
        fs::remove_file(path).expect("Files not deleted!!!");
    }
}
