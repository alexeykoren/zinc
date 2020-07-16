use structopt::StructOpt;

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
    short = "t",
    long = "testcases-file",
    help = "Path to file with testcases"
    )]
    pub testcases: Option<String>,

    #[structopt(
    short = "d",
    long = "tests-dir",
    help = "Path to test directory"
    )]
    pub tests_dir: Option<String>,

    #[structopt(short = "a", long = "assembly-input", help = "Accepts program in VM bytecode instead of .zn source file")]
    pub assembly_input: bool,

    #[structopt(short = "q", long = "quiet", help = "Doesn't show successful tests.")]
    pub quiet: bool,
}
