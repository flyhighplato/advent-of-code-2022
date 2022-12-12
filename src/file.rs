use std::{
    env,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::{Path, PathBuf},
};

use path_clean::PathClean;

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    // let path = absolute_path(filename.as_ref()).unwrap();
    // println!("looking for file: ${:?}", path.clone());

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
