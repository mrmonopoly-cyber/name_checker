use crate::common::DeclinazioneConiugazione;
use crate::db::DB;
use crate::exercise::*;
use clap::{Arg, ArgAction, Command, value_parser};

const DEFAULT_DB_PATH: &str = "db_file.cfg";

pub fn parse_cli_args(exer: &mut ExerciseCheck, db_ref: &mut DB) -> Result<(), String> {
    let command = Command::new("latino")
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbs")
                .short('v')
                .long("verbs")
                .value_name("CONJ")
                .num_args(1..=4)
                .value_parser(value_parser!(usize))
                .help("lexical test for verbs"),
        )
        .arg(
            Arg::new("names")
                .short('n')
                .long("names")
                .value_name("DECL")
                .num_args(1..=4)
                .value_parser(value_parser!(usize))
                .help("lexical test for names"),
        )
        .arg(
            Arg::new("declinazioni")
                .short('d')
                .long("declinazioni")
                .value_name("DECL")
                .num_args(1..=4)
                .value_parser(value_parser!(usize))
                .help("test for declinazioni"),
        )
        .arg(
            Arg::new("coniugazioni")
                .short('c')
                .long("coniugazioni")
                .value_name("CONJ")
                .num_args(1..=4)
                .value_parser(value_parser!(usize))
                .help("test for coniugazioni"),
        )
        .arg(
            Arg::new("all")
            .long("all")
            .action(ArgAction::SetTrue)
            .help("ask all possible question")
        )
        .arg(
            Arg::new("db_file")
                .long("db_file")
                .required(false)
                .value_name("file path")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .help("custom database file"),
        );

    let matches = command.get_matches();
    let mut dec_cons_exec = ConDecListToTest::default();

    pub fn check_field(
        matches: &clap::ArgMatches,
        dec_cons_exec: &mut ConDecListToTest,
        field: &str,
    ) -> bool {
        dec_cons_exec.clear();
        if let Some(decs) = matches.get_many::<usize>(field) {
            for &decs in decs {
                if decs > 0 && decs < usize::from(DeclinazioneConiugazione::__Count) {
                    dec_cons_exec.add_dec_con(DeclinazioneConiugazione::from(decs));
                }
            }
        }
        dec_cons_exec.is_active()
    }
    let db = match matches.get_one::<String>("db_file") {
        Some(path) => db_ref.init(path),
        None => db_ref.init(DEFAULT_DB_PATH),
    };

    if let Err(e) = db {
        return Err(format!("error init db: {e}"));
    }

    if let Some(true) = matches.get_one::<bool>("all"){
        dec_cons_exec.add_dec_con(DeclinazioneConiugazione::I);
        dec_cons_exec.add_dec_con(DeclinazioneConiugazione::II);
        dec_cons_exec.add_dec_con(DeclinazioneConiugazione::III);
        dec_cons_exec.add_dec_con(DeclinazioneConiugazione::IV);

        exer.add_exercise(Exercise::new(ExerciseType::LexicalName, dec_cons_exec));
        exer.add_exercise(Exercise::new(ExerciseType::LexicalVerb, dec_cons_exec));
        exer.add_exercise(Exercise::new(ExerciseType::DeclinaName, dec_cons_exec));
        exer.add_exercise(Exercise::new(ExerciseType::ConiugaVerb, dec_cons_exec));
        return Ok(());
    }

    if check_field(&matches, &mut dec_cons_exec, "names") {
        exer.add_exercise(Exercise::new(ExerciseType::LexicalName, dec_cons_exec));
    }

    if check_field(&matches, &mut dec_cons_exec, "verbs") {
        exer.add_exercise(Exercise::new(ExerciseType::LexicalVerb, dec_cons_exec));
    }

    if check_field(&matches, &mut dec_cons_exec, "declinazioni") {
        exer.add_exercise(Exercise::new(ExerciseType::DeclinaName, dec_cons_exec));
    }

    if check_field(&matches, &mut dec_cons_exec, "coniugazioni") {
        exer.add_exercise(Exercise::new(ExerciseType::ConiugaVerb, dec_cons_exec));
    }


    Ok(())
}
