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
    db_names: [Vec::<Name>; declinazione::Declinazioni::__Count as usize],
    db_verbs: [Vec::<Verbs>; verbs::Coniugazione::__Count as usize],
}

#[derive(Clone, Copy, Default)]
pub struct Id{
    cat: usize,
    idx: usize,
}

impl Id {
    fn new(cat: usize, idx: usize) -> Self{
        Self { cat, idx }
    }
    
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

                let dec =declinazione::Paradigma::new(&ele.latin[0], &ele.latin[1])
                    .get_declinazione();
                let (num, _) = match dec{
                    Ok(d) => d,
                    Err(e) => {
                        println!("error finding declinazione of {ele}: {e}");
                        continue;
                    },
                };

                self.db_names[num].push(ele);
            }
        }
        Ok(())
    }

    pub fn get_name(&self, id: Id) -> Option<&Name> {
        self.db_names[id.cat].get(id.idx)
    }

    pub fn get_verb(&self, id: Id) -> Option<&Verbs> {
        self.db_verbs[id.cat].get(id.idx)
    }

    pub fn get_rand_verb_lat<'a>(&'a self, con: verbs::Coniugazione) -> (Id,verbs::Paradigma<'a>){
        if self.db_verbs.is_empty() {
            (Id::default(), verbs::Paradigma::default())
        }else{
            let con = usize::from(con);
            let con_table = &self.db_verbs[con];
            if con_table.is_empty(){
                return (Id::default(), verbs::Paradigma::default());
            }
            let verb_idx = rng().random_range(0..con_table.len());
            let verb = &con_table[verb_idx];
            let id = Id::new(con, verb_idx);
            let paradigma = verbs::Paradigma::new(&verb.latin);

            (id, paradigma)
        }

    }

    pub fn get_rand_name_lat<'a>(&'a self, dec: declinazione::Declinazioni) -> 
        (Id, declinazione::Paradigma<'a>){
            if self.db_names.is_empty(){
                (Id::default(),declinazione::Paradigma::default())
            }else{
                let dec = usize::from(dec);
                let dec_table = &self.db_names[dec];
                if dec_table.is_empty() {
                    return (Id::default(),declinazione::Paradigma::default());
                }
                let name_idx = rng().random_range(0..dec_table.len());
                let name = &dec_table[name_idx];
                let id = Id::new(dec, name_idx);
                let paradigma = declinazione::Paradigma::new(&name.latin[0], &name.latin[1]);

                (id,paradigma)
            }
    }

    pub fn get_rand_verb_it(&self, con: verbs::Coniugazione) -> (Id, &str){
        if self.db_verbs.is_empty() {
            (Id::default(),"")
        }else{
            let con = usize::from(con);
            let con_table = &self.db_verbs[con];
            if con_table.is_empty(){
                return (Id::default(), "");
            }
            let verb_idx = rng().random_range(0..self.db_verbs.len());
            let verb = &con_table[verb_idx];
            (Id::new(con, verb_idx), &verb.italian)
        }
    }

    pub fn get_rand_name_it(&self, dec: declinazione::Declinazioni) -> (Id, &str){
        if self.db_names.is_empty(){
            (Id::default(), "")
        }
        else{
            let dec = usize::from(dec);
            let dec_table = &self.db_names[dec];
            if dec_table.is_empty() {
                return (Id::default(),"");
            }
            let name_idx = rng().random_range(0..self.db_names.len());
            let name = &dec_table[name_idx];
            (Id::new(dec, name_idx), &name.italian)
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
