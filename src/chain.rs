use crate::archive::Archive;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Default)]
pub struct Chain {
    chain: Vec<Archive>,
}

impl Chain {
    pub fn new() -> Self {
        Chain { chain: Vec::new() }
    }

    pub fn add<P: AsRef<Path>>(&mut self, path: P) {
        match Archive::open(path) {
            Ok(v) => self.chain.insert(0, v),
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn read(&mut self, filename: &str) -> Result<Vec<u8>, Error> {
        for mut archive in &mut self.chain.iter_mut() {
            if let Ok(file) = archive.open_file(filename) {
                let mut buf: Vec<u8> = vec![0; file.size() as usize];

                match file.read(&mut archive, &mut buf) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{} {:#?}", e, archive);
                    }
                }

                return Ok(buf);
            }
        }

        Err(Error::new(
            ErrorKind::NotFound,
            "File not found in mpq chain",
        ))
    }

    pub fn read_to_string(&mut self, filename: &str) -> Result<String, Error> {
        match self.read(filename) {
            Ok(buf) => match String::from_utf8(buf) {
                Ok(v) => Ok(v),
                Err(_) => Err(Error::new(ErrorKind::InvalidData, "Utf8Error")),
            },
            Err(e) => Err(e),
        }
    }
}
