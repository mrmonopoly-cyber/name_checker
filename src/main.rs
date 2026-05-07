mod db;
mod declinazione;
mod exercise;
mod cli;
mod verbs;

use std::io::Write;
use std::process::exit;
use self::db::DB;
use self::exercise::ExeRes;

fn main() {
    let mut user_input = String::new();
    let mut exercise = exercise::ExerciseCheck::default();
    let mut score = ExeRes::default();
    let mut db = DB::default();
    if let Err(e) = cli::parse_cli_args(&mut exercise, &mut db){
        println!("error parsing cli input: {e}");
        exit(1);
    }
    exercise.add_db(&db);

    if exercise.num_exercise() != 0
    {
        loop{
            exercise.question(&mut user_input);
            print!("{}",user_input);
            if let Err(e) = ::std::io::stdout().flush(){
                println!("error flush stdout: {e}");
            }

            if let Err(e) = get_user_input(&mut user_input){
                println!("error reading user input {e}");
                continue;
            }

            if user_input == exercise::QUIT_COMMAND {
                break;
            }

            match exercise.answer(&user_input) {
                true => score.success(),
                false => score.fail(),
            }
        }
    }

    println!("{}", score);
}

fn get_user_input(user_input: &mut String) -> Result<(), String>{
    user_input.clear();
    match ::std::io::stdin().read_line(user_input){
        Err(e) => {
            Err(format!("error reading stdin: {e}"))
        },
        Ok(_) => {
            user_input.pop();
            Ok(())
        },
    }
}

