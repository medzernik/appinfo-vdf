pub mod vdf;

use std::fs::File;
use std::io;
use std::io::Read;
use vdf::reader::read;
pub use vdf::VDF;

pub fn read_file(path: &str) -> io::Result<VDF> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (_, vdf) = read(&buffer)?;
    Ok(vdf)
}
