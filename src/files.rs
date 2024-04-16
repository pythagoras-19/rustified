use std::fs::{File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct MyPermissions {
    readonly: bool,
}

#[derive(Serialize, Deserialize)]
struct MyMetadata {
    is_dir: bool,
    is_file: bool,
    permissions: MyPermissions,
    modified: std::time::SystemTime,
    accessed: std::time::SystemTime,
    created: std::time::SystemTime,
}

#[derive(Serialize, Deserialize)]
struct PathData {
    path_ancestors: Vec<PathBuf>,
    path_metadata: Option<MyMetadata>,
    path_components: Vec<String>,
}

pub fn open() -> Result<()> {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    let ancestors = path.ancestors().map(Path::to_path_buf).collect();
    let metadata = match path.metadata() {
        Ok(metadata) => Some(MyMetadata {
            is_dir:metadata.is_dir(),
            is_file: metadata.is_file(),
            permissions: MyPermissions {
                readonly: metadata.permissions().readonly(),
            },
            modified: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH),
            accessed: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH),
            created: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH),
        }),
        Err(e) => None,
    };
    let components = path.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();

    let pd = PathData {
        path_ancestors: ancestors,
        path_metadata: metadata,
        path_components: components,
    };

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}!", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{} \n", display, s),
    }

    println!();

    println!("===JSON DATA===");
    // Serialize it to a JSON string.
    let serialized_pd = serde_json::to_string(&pd)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", serialized_pd);

    Ok(())

    // `file` goes out of scope, so the "hello.txt" file gets closed automagically
}