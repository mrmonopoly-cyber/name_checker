mod db;
mod declinazione;
mod exercise;
mod cli;
mod verbs;
mod common;

use std::io::Write;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use db::DB;
use exercise::ExeRes;

static RUNNING: AtomicBool = AtomicBool::new(true);

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

    let (tx,rx) : (Sender<String>, Receiver<String>) = mpsc::channel();

    ::std::thread::spawn(move ||{
        let mut user_input = String::new();

        loop{
            user_input.clear();

            if let Err(e) = get_user_input(&mut user_input){
                println!("error reading user input {e}");
                break;
            }

            if let Err(e) = tx.send(user_input.clone()){
                println!("error sending user input {e}");
                continue;
            }
        };
    });

    ctrlc::set_handler(move ||{
        RUNNING.store(false, Ordering::Relaxed);
    }).expect("error setting Ctrl-C handler");

    if exercise.num_exercise() != 0
    {
        while RUNNING.load(Ordering::Relaxed){
            exercise.question(&mut user_input);
            print!("{}",user_input);
            if let Err(e) = ::std::io::stdout().flush(){
                println!("error flush stdout: {e}");
            }

            loop{
                match rx.recv_timeout(Duration::from_millis(100)){
                    Ok(user_input) => {
                        if user_input != exercise::QUIT_COMMAND {
                            match exercise.answer(&user_input) {
                                true => score.success(),
                                false => score.fail(),
                            }
                        }
                        else{
                            RUNNING.store(false, Ordering::Relaxed);
                        }

                        break;
                    },
                    Err(mpsc::RecvTimeoutError::Disconnected) =>{
                        println!("Disconnected");
                        break;
                    },
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        if !RUNNING.load(Ordering::Relaxed) {
                            println!();
                            break;
                        }
                    },
                }
            }
        }
        println!("{}", score);
        let _ =::std::io::stdout().flush();
    }

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

