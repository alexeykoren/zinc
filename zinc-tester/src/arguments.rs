use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "zinc-tester",
    about = "integration test runner for zinc framework"
)]
pub struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity."
    )]
    pub verbosity: usize,

    #[structopt(
        short = "p",
        long = "proof-check",
        help = "Performs proof-check for every test case"
    )]
    pub proof_check: bool,

    #[structopt(
    short = "f",
    long = "test-file",
    help = "Path to test file"
    )]
    pub testfile: Option<PathBuf>,

    #[structopt(
        short = "t",
        long = "testcases-file",
        help = "Path to file with testcases"
    )]
    pub testcases: Option<PathBuf>,

    #[structopt(
        short = "d",
        long = "tests-dir",
        help = "Path to test directory"
    )]
    pub tests_dir: Option<String>,

    #[structopt(
        short = "q",
        long = "quiet",
        help = "Doesn't show successful tests."
    )]
    pub quiet: bool,
}
