mod db;
mod declinazione;
mod exercise;

use clap::Parser;
use std::process::exit;

use db::DB;
use self::declinazione::PRIMA_DECLINAZIONE;
use self::exercise::Exercise;

enum TestType{
    Lexical,
    PrimaDeclinazione,
}

struct ProgInput{
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

fn main() {
    let args = Args::parse();
    let mut input = ProgInput::default();

    if let Some(s) = args.test_type{
        match s.as_str() {
            "names"  => input.test_type = TestType::Lexical,
            "I dec" => input.test_type = TestType::PrimaDeclinazione,
            _ => input.test_type = TestType::Lexical,
        }
    }

    match input.test_type{
        TestType::Lexical => create_and_run_lex_exercise(&input),
        TestType::PrimaDeclinazione => PRIMA_DECLINAZIONE.run_exercise(),
    }
}

fn create_and_run_lex_exercise(input: &ProgInput)
{
    let db = match DB::new(::std::path::Path::new(input.db_file))
    {
        Ok(db) => db,
        Err(e) => {
            println!("error creating db: {e}");
            exit(1);
        },
    };
    db.run_exercise()
}
