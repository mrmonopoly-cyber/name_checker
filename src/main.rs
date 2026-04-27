mod db;

use std::io::Write;
use std::process::exit;
use rand::{self, RngExt};

use db::DB;

struct ProgInput{
   pub db_file: &'static str, 
}

impl Default for ProgInput{
    fn default() -> Self {
        Self { db_file: "db_file.txt"}
    }
}

fn main() {
    let input = ProgInput::default();

    let db = match DB::new(::std::path::Path::new(input.db_file))
    {
        Ok(db) => db,
        Err(e) => {
            println!("error creating db: {e}");
            exit(1);
        },
    };

    let mut rng = rand::rng();

    let mut user_input = String::new();

    loop{
        let n = rng.random_range(0..db.len());
        let name_ref = db.at(n).expect("invalid range");

        print!("tell me the latin for of {} (use , as separator): ", name_ref.italian);
        std::io::stdout().flush().unwrap();

        user_input.clear();
        match ::std::io::stdin().read_line(&mut user_input){
            Err(e) => println!("error reading stdin: {e}"),
            Ok(_) => {
                user_input.pop();
                let mut split = user_input.split(',');
                check_caso(split.next(), name_ref, db::Casi::Nominativo);
                print!(", ");
                check_caso(split.next(), name_ref, db::Casi::Genitivo);
                println!();
            },
        }
    }

}


fn check_caso(si: Option<&str>, name_ref: &db::Name, caso: db::Casi)
{
    match si
    {
        Some(s) => {
            match s.trim().trim_end() == name_ref.latin[caso as usize]{
                true => {
                    print!("correct {}", caso);
                    std::io::stdout().flush().unwrap();
                },
                false => println!("incorrect {caso}: given {}, expected {}",
                    s, name_ref.latin[caso as usize]),
            }
        },
        None => println!("missing input"),
    }
}
