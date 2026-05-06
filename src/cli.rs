use clap::Parser;

pub enum TestType{
    Lexical,
    Verbs,
    Declination,


    #[allow(nonstandard_style)]
    __Num_Test
}

#[derive(Default)]
pub struct ProgInput{
    pub test_type : usize,
    pub db_file: String, 
    pub dec_tested: usize,
}

const DEFAULT_DB_PATH : &str = "db_file.txt";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// test verbs declination
    #[arg(short, long)]
    verbs: Option<bool>,

    /// test name memory 
    #[arg(short, long)]
    names: Option<bool>,

    /// test name declination
    #[arg(short, long)]
    declinazione: Option<Vec<usize>>,

    /// custom db file
    #[arg(short, long)]
    db_file: Option<String>,
}

pub fn parse_cli_args() -> ProgInput{
    let args = Args::parse();
    let mut res = ProgInput::default();

    if Some(true) == args.verbs {
        res.test_type |= 1 << usize::from(TestType::Verbs);
    }

    if Some(true) == args.names{
        res.test_type |= 1 << usize::from(TestType::Lexical);
    }

    if let Some(dec_list) = args.declinazione{
        res.test_type |= 1 << usize::from(TestType::Declination);
        for dec in dec_list{
            res.dec_tested |= 1 << dec;
        }
    }

    match args.db_file{
        Some(s) => res.db_file = s,
        None => res.db_file.push_str(DEFAULT_DB_PATH),
    };

    res
}

impl From<TestType> for usize{
    fn from(value: TestType) -> Self {
        value as Self
    }
}

impl From<usize> for TestType{
    fn from(value: usize) -> Self {
        match value{
            0 => TestType::Lexical,
            1 => TestType::Verbs,
            2 => TestType::Declination,
            _ => TestType::Lexical,
        }
    }
}
