use std::fmt::Display;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use rand::{rng, RngExt};

use crate::verbs;
use crate::declinazione;

pub enum DBError {
    InvalidLinePattern (usize),
    IO (::std::io::Error),
    
}

#[derive(Debug, Clone)]
pub struct Name {
    pub italian: String,
    pub latin: [String;2],
}

#[derive(Debug, Clone)]
pub struct Verbs{
    pub italian: String,
    pub latin: [String;5],
}

#[derive(Default)]
pub struct DB{
    db_names: Vec::<Name>,
    db_verbs: Vec::<Verbs>,
    it_i: usize,
}

impl DB{
    pub fn init(&mut self, path : &str) -> Result<(), DBError>
    {
        macro_rules! check_split {
            ($split:expr, $error: expr) => {
                match $split{
                    Some(s) => s,
                    None => return $error,
                }

            };
        }
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

                self.db_names.push(ele);
            }
        }
        Ok(())
    }

    pub fn get_name(&self, id: usize) -> Option<&Name> {
        self.db_names.get(id)
    }

    pub fn get_verb(&self, id: usize) -> Option<&Verbs> {
        self.db_verbs.get(id)
    }

    pub fn get_rand_verb_lat<'a>(&'a self, _con: verbs::Coniugazione)
        -> (usize,verbs::Paradigma<'a>){
        if self.db_verbs.is_empty() {
            (0,verbs::Paradigma::default())
        }else{
            let verb_idx = rng().random_range(0..self.db_verbs.len());
            let verb = &self.db_verbs[verb_idx];
            (verb_idx, verbs::Paradigma::new(&verb.latin))
        }

    }

    pub fn get_rand_name_lat<'a>(&'a self, _dec: declinazione::Declinazioni) ->
        (usize, declinazione::Paradigma<'a>){
            if self.db_names.is_empty(){
                (0,declinazione::Paradigma::default())
            }else{
                let name_idx = rng().random_range(0..self.db_names.len());
                let name = &self.db_names[name_idx];
                (name_idx, declinazione::Paradigma::new(&name.latin[0], &name.latin[1]))
            }
    }

    pub fn get_rand_verb_it(&self, _con: verbs::Coniugazione) -> (usize, &str){
        if self.db_verbs.is_empty() {
            (0,"")
        }else{
            let verb_idx = rng().random_range(0..self.db_verbs.len());
            let verb = &self.db_verbs[verb_idx];
            (verb_idx, &verb.italian)
        }
    }

    pub fn get_rand_name_it(&self, _dec: declinazione::Declinazioni) -> (usize, &str){
        if self.db_names.is_empty(){
            (0, "")
        }
        else{
            let name_idx = rng().random_range(0..self.db_names.len());
            let name = &self.db_names[name_idx];
            (name_idx, &name.italian)
        }
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
        if self.it_i < self.db_names.len(){
            let name = self.db_names[self.it_i].clone();
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
