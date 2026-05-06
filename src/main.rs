mod db;
mod declinazione;
mod exercise;
mod cli;
mod verbs;

use std::process::exit;

use rand::RngExt;

use self::cli::TestType;
use self::db::DB;


fn main() {
    let mut rng = rand::rng();
    let mut user_input = String::new();
    let input = cli::parse_cli_args();
    let db = match DB::new(&input.db_file){
        Ok(db) => db,
        Err(e) => {
            println!("error creating db: {e}");
            exit(1);
        },
    };

    if input.test_type != 0
    {
        loop{
            let test : TestType = {
                let mut possible = 0;
                while (input.test_type & 1 << possible) == 0 {
                    possible = rng.random_range(0..usize::from(cli::TestType::__Num_Test));
                }
                TestType::from(possible)
            };

            match test {
                TestType::Lexical => {
                    todo!()
                },
                TestType::Verbs => {
                        let _verb = db.get_rand_verb();
                },
                TestType::Declination => {
                        let _name = db.get_rand_name();
                },
                TestType::__Num_Test => unreachable!(),
            }

            user_input.clear();
            match ::std::io::stdin().read_line(&mut user_input){
                Err(e) => {
                    println!("error reading stdin: {e}");
                    continue;
                },
                Ok(_) => {
                    user_input.pop();
                },
            }

            if user_input == exercise::QUIT_COMMAND {
                break;
            }
        }
    }


}
