mod db;
mod declinazione;
mod exercise;
mod cli;
mod verbs;

use std::process::exit;

use db::DB;
use self::declinazione::PRIMA_DECLINAZIONE;
use self::exercise::Exercise;

fn main() {
    let input = cli::parse_cli_args();
    let res = match input.test_type{
        cli::TestType::Lexical => create_and_run_lex_exercise(&input),
        cli::TestType::PrimaDeclinazione => PRIMA_DECLINAZIONE.run_exercise(),
        cli::TestType::Verbs => todo!(),
    };

    println!("{res}");
}

fn create_and_run_lex_exercise(input: &cli::ProgInput) -> exercise::ExeRes
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
