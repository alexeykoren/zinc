//!
//! The Zinc tester data.
//!

use std::str::FromStr;

use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestCase {
    pub case: String,
    #[serde(default)]
    pub should_panic: bool,
    #[serde(default)]
    pub ignore: bool,
    pub input: JsonValue,
    pub expect: JsonValue,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestData {
    pub cases: Vec<TestCase>,
    #[serde(default)]
    pub ignore: bool,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "parsing: {}", _0)]
    Parsing(serde_json::Error),
    #[fail(display = "opening: {}", _0)]
    Opening(io::Error),
    #[fail(display = "metadata: {}", _0)]
    Metadata(io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
}

static LINE_PREFIX: &str = "//#";

impl FromStr for TestData {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let json = string
            .lines()
            .filter_map(|line| {
                if line.starts_with(LINE_PREFIX) {
                    Some(&line[LINE_PREFIX.len()..])
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join("");

        serde_json::from_str(&json).map_err(Error::Parsing)
    }
}
impl TryFrom<&PathBuf> for TestData {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(path).map_err(Error::Opening)?;
        let size = file.metadata().map_err(Error::Metadata)?.len() as usize;
        let mut json = String::with_capacity(size);
        file.read_to_string(&mut json).map_err(Error::Reading)?;
        serde_json::from_str(&json).map_err(Error::Parsing)
    }
}

