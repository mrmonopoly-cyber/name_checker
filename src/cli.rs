use clap::Parser;

pub enum TestType{
    Lexical,
    Verbs,
    PrimaDeclinazione,
}

pub struct ProgInput{
    pub db_file: &'static str, 
    pub test_type: TestType,
}

impl Default for ProgInput{
    fn default() -> Self {
        Self { db_file: "db_file.txt", test_type: TestType::Lexical}
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    test_type: Option<String>,
}

pub fn parse_cli_args() -> ProgInput{
    let args = Args::parse();
    let mut res = ProgInput::default();

    if let Some(s) = args.test_type{
        match s.as_str() {
            "names"  => res.test_type = TestType::Lexical,
            "verbs" => res.test_type = TestType::Verbs,
            "I dec" => res.test_type = TestType::PrimaDeclinazione,
            _ => res.test_type = TestType::Lexical,
        }
    }


    res
}
