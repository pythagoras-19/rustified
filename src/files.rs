use std::fs::{File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Result;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

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

pub fn entry() {
    open().expect("TODO: panic message");
    write();
}
fn open() -> Result<()> {
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

fn write() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}