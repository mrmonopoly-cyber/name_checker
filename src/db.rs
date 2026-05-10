use std::fmt::Display;
use std::io::{self, BufRead, BufReader, Lines};
use std::fs::File;
use std::path::Path;
use rand::{rng, RngExt};

use crate::common::DeclinazioneConiugazione;
use crate::verbs;
use crate::declinazione;

pub enum DBError {
    InvalidLinePattern (String),
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
    db_names: [Vec::<Name>; DeclinazioneConiugazione::__Count as usize],
    db_verbs: [Vec::<Verbs>; DeclinazioneConiugazione::__Count as usize],
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

#[derive(PartialEq)]
enum SectionCategory{
    Names,
    Verbs,
    None,
}

enum LineType<'a> {
    Section(SectionCategory),
    Data(&'a str),
    Comment,
    Empty,
    CloseSection,
}

macro_rules! check_split {
    ($split:expr, $error: expr) => {
        match $split{
            Some(s) => s,
            None => return $error,
        }

    };
}

impl DB{
    fn parse_line<'a>(&mut self, line: &'a str) -> LineType<'a> {
        if line.starts_with('#') {
            LineType::Comment
        }
        else if line.is_empty(){
            LineType::Empty
        }

        else if line == "Names{" {
            LineType::Section(SectionCategory::Names)
        }

        else if line == "Verbs{" {
            LineType::Section(SectionCategory::Verbs)
        }

        else if line == "}"{
            LineType::CloseSection
        }
        else{
            LineType::Data(line)
        }
    }

    fn parse_section(&mut self, lines: &mut Lines<BufReader<File>>) -> Result<bool, DBError>{
        let mut section = SectionCategory::None;
        let mut italian = String::new();
        let mut latins = String::new();

        while let Some(Ok(untrim_line)) = lines.next(){
            let line = untrim_line.trim();
            match self.parse_line(line) {
                LineType::Section(section_category) => {
                    if section == SectionCategory::None {
                        section = section_category;
                    }
                    else if let Err(e) = self.parse_section(lines){
                        return Err(e);
                    }
                },
                LineType::Data(s) => {
                    match section {
                        SectionCategory::Names => {
                            let words : Vec<&str> = line.split(':').collect();
                            if words.len() != 2 {
                                return Err(DBError::InvalidLinePattern(untrim_line));
                            }
                            italian.clear();
                            latins.clear();

                            italian.push_str(words[0]);
                            latins.push_str(words[1]);

                            let mut split = latins.split(',');

                            let lat_nom = check_split!(
                                split.next(), Err(DBError::InvalidLinePattern(untrim_line)));
                            let lat_gen = check_split!(
                                split.next(), Err(DBError::InvalidLinePattern(untrim_line)));

                            let ele = Name{
                                italian: italian.to_string(),
                                latin: [
                                    lat_nom.to_string(),
                                    lat_gen.to_string()
                                ],
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
                        },
                        SectionCategory::Verbs => {
                            let words : Vec<&str> = line.split(':').collect();
                            if words.len() != 2 {
                                return Err(DBError::InvalidLinePattern(untrim_line));
                            }
                            italian.clear();
                            latins.clear();

                            italian.push_str(words[0]);
                            latins.push_str(words[1]);

                            let mut split = latins.split(',');

                            let ind_pre_1_pers = check_split!(
                                split.next(), Err(DBError::InvalidLinePattern(untrim_line)));
                            let ind_pre_2_pers= check_split!(
                                split.next(), Err(DBError::InvalidLinePattern(untrim_line)));
                            let perfetto = check_split!(
                                split.next(), Err(DBError::InvalidLinePattern(untrim_line)));
                            let supino = check_split!(
                                split.next(), Err(DBError::InvalidLinePattern(untrim_line)));
                            let infinito = check_split!(
                                split.next(), Err(DBError::InvalidLinePattern(untrim_line)));

                            let ele = Verbs{
                                italian: italian.to_string(),
                                latin: [
                                    ind_pre_1_pers.to_string(),
                                    ind_pre_2_pers.to_string(),
                                    perfetto.to_string(),
                                    supino.to_string(),
                                    infinito.to_string()
                                ],
                            };

                            let con = verbs::Paradigma::new(&ele.latin)
                                .get_coniugazione();

                            let num = match con{
                                Ok(c) => c,
                                Err(e) => {
                                    println!("error finding Coniugazione of {ele}: {e}");
                                    continue;
                                },
                            };

                            self.db_verbs[num].push(ele);
                        },
                        SectionCategory::None => {
                            println!("WARNING: {s} is not in a category and will be lost");
                        },
                    }
                },
                LineType::Comment | LineType::Empty => (),
                LineType::CloseSection => return Ok(false),
            }
        };

        Ok(true)

    }

    pub fn init(&mut self, path : &str) -> Result<(), DBError>
    {
        let mut lines = match read_lines(path){
            Ok(l) => l,
            Err(e) => {
                return Err(DBError::IO(e));
            },
        };

        loop{
            match self.parse_section(&mut lines) {
                Ok(true) => return Ok(()),
                Err(e) => return Err(e),
                _ => (),
            }
        }
    }

    pub fn get_name(&self, id: Id) -> Option<&Name> {
        self.db_names[id.cat].get(id.idx)
    }

    pub fn get_verb(&self, id: Id) -> Option<&Verbs> {
        self.db_verbs[id.cat].get(id.idx)
    }

    pub fn get_rand_verb_lat<'a>(&'a self, con: DeclinazioneConiugazione) -> (Id,verbs::Paradigma<'a>){
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

    pub fn get_rand_name_lat<'a>(&'a self, dec: DeclinazioneConiugazione) -> 
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

    pub fn get_rand_verb_it(&self, con: DeclinazioneConiugazione) -> (Id, &str){
        if self.db_verbs.is_empty() {
            (Id::default(),"")
        }else{
            let con = usize::from(con);
            let con_table = &self.db_verbs[con];
            if con_table.is_empty(){
                return (Id::default(), "");
            }
            let verb_idx = rng().random_range(0..self.db_verbs[con].len());
            let verb = &con_table[verb_idx];
            (Id::new(con, verb_idx), &verb.italian)
        }
    }

    pub fn get_rand_name_it(&self, dec: DeclinazioneConiugazione) -> (Id, &str){
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

impl Display for Verbs{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:[{},{},{},{},{}]", self.italian,
            self.latin[0],
            self.latin[1],
            self.latin[2],
            self.latin[3],
            self.latin[4],
            )
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
