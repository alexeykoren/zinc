//!
//! The Zinc tester file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use crate::directory::TEST_FILE_EXTENSION_DEFAULT;

use failure::Fail;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestFile {
    pub code: String,
    pub assembly: bool,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "opening: {}", _0)]
    Opening(io::Error),
    #[fail(display = "metadata: {}", _0)]
    Metadata(io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
}

impl TryFrom<&PathBuf> for TestFile {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(path).map_err(Error::Opening)?;
        let size = file.metadata().map_err(Error::Metadata)?.len() as usize;
        // TODO decide file extension
        let assembly = if path.extension().unwrap().to_str().unwrap() == TEST_FILE_EXTENSION_DEFAULT { false } else { true };
        let mut string = String::with_capacity(size);
        file.read_to_string(&mut string).map_err(Error::Reading)?;
        Ok(Self { code: string, assembly: assembly })
    }
}
