use crate::verbs::Coniugazione;
use crate::declinazione::Declinazioni;
use clap::{value_parser, Arg, ArgAction, Command};
use crate::exercise::*;
use crate::db::DB;

const DEFAULT_DB_PATH : &str = "db_file.cfg";

pub fn parse_cli_args(exer: &mut ExerciseCheck, db_ref: &mut DB) -> Result<(), String>{
    let command = Command::new("latino")
        .arg_required_else_help(true)
        .arg(Arg::new("verbs")
            .short('v')
            .long("verbs")
            .action(ArgAction::SetTrue)
            .help("lexical test for verbs")
        )
        .arg(Arg::new("names")
            .short('n')
            .long("names")
            .action(ArgAction::SetTrue)
            .help("lexical test for names")
        )
        .arg(Arg::new("declinazioni")
            .short('d')
            .long("declinazioni")
            .value_name("DECL")
            .num_args(1..=4)
            .value_parser(value_parser!(usize))
            .help("test for declinazioni")
        )
        .arg(Arg::new("coniugazioni")
            .short('c')
            .long("coniugazioni")
            .value_name("CONJ")
            .num_args(1..=4)
            .value_parser(value_parser!(usize))
            .help("test for coniugazioni")
        )
        .arg(Arg::new("db_file")
            .long("db_file")
            .required(false)
            .value_name("file path")
            .value_parser(value_parser!(String))
            .action(ArgAction::Set)
            .help("custom database file")
        );
        
    let matches = command.get_matches();
    let mut lexical = ([const {LexicalType::Verbs};2],0);

    if matches.get_flag("verbs"){
        lexical.0[lexical.1] = LexicalType::Verbs;
        lexical.1+=1;
    }

    if matches.get_flag("names"){
        lexical.0[lexical.1] = LexicalType::Names;
        lexical.1+=1;
    }

    if lexical.1 > 0{
        exer.add_exercise(Exercise::Lexical((Some(lexical.0), lexical.1)));
    }

    if let Some(decs) = matches.get_many::<usize>("declinazioni"){
        let mut declinazioni = ([const {Declinazioni::Prima};4],0);
        for &dec in decs{
            if dec < usize::from(Declinazioni::__Count){
                declinazioni.0[declinazioni.1] = Declinazioni::from(dec);
                declinazioni.1+=1;
            }
        }
        exer.add_exercise(Exercise::DeclinaName((Some(declinazioni.0), declinazioni.1)));
    }

    if let Some(cons) = matches.get_many::<usize>("coniugazioni"){
        let mut coniugazioni = ([const {Coniugazione::I};4],0);
        for &con in cons{
            if con > 0 && con < usize::from(Declinazione::__Count){
                coniugazioni.0[coniugazioni.1] = Coniugazione::from(con);
                coniugazioni.1+=1;
            }
        }
        exer.add_exercise(Exercise::ConiugaVerb((Some(coniugazioni.0), coniugazioni.1)));
    }

    let db = match matches.get_one::<String>("db_file"){
        Some(path) =>db_ref.init(path),
        None => db_ref.init(DEFAULT_DB_PATH),
    };

    if let Err(e) = db{
        return Err(format!("error init db: {e}"));
    }

    Ok(())
}
