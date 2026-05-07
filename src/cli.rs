use crate::verbs::Coniugazione;
use crate::declinazione::Declinazioni;
use clap::Parser;
use crate::exercise::*;
use crate::db::DB;

const DEFAULT_DB_PATH : &str = "db_file.txt";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// test verbs memory 
    #[arg(short, long)]
    verbs: Option<bool>,

    /// test name memory 
    #[arg(short, long)]
    names: Option<bool>,

    /// test name declination
    #[arg(short, long)]
    declinazioni: Option<Vec<usize>>,

    /// test verbs coniugazions 
    #[arg(short, long)]
    conniugazini: Option<Vec<usize>>,

    /// custom db file
    #[arg(long)]
    db_file: Option<String>,
}

pub fn parse_cli_args(exer: &mut ExerciseCheck) -> Result<(), String>{
    let args = Args::parse();
    let mut coniugazioni = ([const {Coniugazione::I};4],0);
    let mut declinazioni = ([const {Declinazioni::Prima};4],0);
    let mut lexical = ([const {LexicalType::Verbs};2],0);

    if Some(true) == args.verbs {
        lexical.0[lexical.1] = LexicalType::Verbs;
        lexical.1+=1;
    }

    if Some(true) == args.names{
        lexical.0[lexical.1] = LexicalType::Names;
        lexical.1+=1;
    }

    if let Some(dec_list) = args.declinazioni{
        for dec in dec_list{
            if dec < usize::from(Declinazione::__Count){
                declinazioni.0[declinazioni.1] = Declinazioni::from(dec);
                declinazioni.1+=1;
            }
        }
    }

    if let Some(con_list) = args.conniugazini{
        for con in con_list {
            if con < usize::from(Declinazione::__Count){
                coniugazioni.0[coniugazioni.1] = Coniugazione::from(con);
                coniugazioni.1+=1;
            }
        }
    }

    let db = match args.db_file {
        Some(path) => DB::new(&path),
        None => DB::new(DEFAULT_DB_PATH),
    };

    match db {
        Ok(db) => exer.add_db(db),
        Err(e) => return Err(format!("error init db: {e}")),
    }
    

    if coniugazioni.1 > 0 {
        exer.add_exercise(Exercise::ConiugaVerb((Some(coniugazioni.0), coniugazioni.1)));
        
    }

    if declinazioni.1 > 0 {
        exer.add_exercise(Exercise::DeclinaName((Some(declinazioni.0), declinazioni.1)));
    }

    if lexical.1 > 0{
        exer.add_exercise(Exercise::Lexical((Some(lexical.0), lexical.1)));
    }


    Ok(())
}
