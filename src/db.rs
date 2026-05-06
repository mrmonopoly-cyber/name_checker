use std::fmt::{Display};
use std::io::{self, BufRead, Write};
use std::fs::File;
use std::path::Path;
use rand::{self, RngExt};

use crate::declinazione;
use crate::exercise::{ExeRes, Exercise, QUIT_COMMAND};

pub enum DBError {
    InvalidLinePattern (usize),
    IO (::std::io::Error),
    
}

#[derive(Debug, Clone)]
pub struct Name {
    pub italian: String,
    pub latin: [String;2],
}

pub struct DB{
    db: Vec::<Name>,
    it_i: usize,
}

impl DB{
    pub fn new(path : &std::path::Path) -> Result<Self, DBError>
    {
        macro_rules! check_split {
            ($split:expr, $error: expr) => {
                match $split{
                    Some(s) => s,
                    None => return $error,
                }

            };
        }
        let mut res = Self{db: Vec::new(), it_i:0};
        let mut italian = String::new();
        let mut latins = String::new();
        let lines = match read_lines(path){
            Ok(l) => l,
            Err(e) => {
                return Err(DBError::IO(e));
            },
        };

        for (idx, line) in lines.map_while(Result::ok).enumerate() {
            let error = Err(DBError::InvalidLinePattern(idx));
            if !line.starts_with('#') && !line.is_empty() {
                let words : Vec<&str> = line.split(':').collect();
                if words.len() != 2 {
                    return error;
                }

                italian.clear();
                latins.clear();

                italian.push_str(words[0]);
                latins.push_str(words[1]);

                let mut split = latins.split(',');

                let lat_nom = check_split!(split.next(), error);
                let lat_gen = check_split!(split.next(), error);

                let ele = Name{
                    italian: italian.to_string(),
                    latin: [lat_nom.to_string(), lat_gen.to_string()],
                };

                res.db.push(ele);
            }
        }

        Ok(res)
    }
    fn len(&self) -> usize{
        self.db.len()
    }

    fn at(&'_ self, i: usize) -> Option<&'_ Name>{
        self.db.get(i)
    }

    fn check_caso(si: Option<&str>, name_ref: &Name, caso: declinazione::Casi, res: &mut ExeRes)
    {
        match si
        {
            Some(s) => {
                match s.trim().trim_end() == name_ref.latin[caso as usize]{
                    true => {
                        res.success();
                        print!("correct {}", caso);
                        std::io::stdout().flush().unwrap();
                    },
                    false => {
                        res.fail();
                        println!("incorrect {caso}: given {}, expected {}",
                        s, name_ref.latin[caso as usize]);
                    },
                }
            },
            None => println!("missing input"),
        }
    }
}

impl Exercise for DB{
    fn run_exercise(&self) -> ExeRes{
        let mut res = ExeRes::default();
        let mut rng = rand::rng();

        let mut user_input = String::new();

        loop{
            let n = rng.random_range(0..self.len());
            let name_ref = self.at(n).expect("invalid range");

            print!("tell me the latin for of {} (use , as separator): ", name_ref.italian);
            std::io::stdout().flush().unwrap();

            user_input.clear();
            match ::std::io::stdin().read_line(&mut user_input){
                Err(e) => println!("error reading stdin: {e}"),
                Ok(_) => {
                    user_input.pop();
                    if user_input == QUIT_COMMAND{
                        break;
                    }
                    let mut split = user_input.split(',');
                    DB::check_caso(split.next(), name_ref, declinazione::Casi::Nominativo, &mut res);
                    print!(", ");
                    DB::check_caso(split.next(), name_ref, declinazione::Casi::Genitivo, &mut res);
                    println!();
                },
            }
        }

        res
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Iterator for DB{
    type Item = Name;

    fn next(&mut self) -> Option<Self::Item> {
        if self.it_i < self.db.len(){
            let name = self.db[self.it_i].clone();
            self.it_i+=1;
            return Some(name);
        }
        None
    }
}

impl Display for Name{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:[{},{}]", self.italian, self.latin[0], self.latin[1])
    }
}

impl Display for DBError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::InvalidLinePattern(n) => write!(f,"invalid pattern at line {n}"),
            DBError::IO(error) => write!(f, "{error}"),
        }
    }
    // add code here
}
