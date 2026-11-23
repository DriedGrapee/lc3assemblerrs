use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
};

pub fn open_buffered(p: impl AsRef<Path>) -> io::Result<BufReader<File>> {
    File::open(p).map(BufReader::new)
}
pub fn create_buffered(p: impl AsRef<Path>) -> io::Result<BufWriter<File>> {
    File::create(p).map(BufWriter::new)
}
