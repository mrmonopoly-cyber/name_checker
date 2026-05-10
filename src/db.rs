use rand::{RngExt, rng};
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

use crate::common::{DeclinazioneConiugazione, GeneralPradigma};
use crate::declinazione;
use crate::verbs;

pub enum DBError {
    InvalidLinePattern(String),
    IO(::std::io::Error),
}

#[derive(Default)]
pub struct NameEntry {
    italian: String,
    latin: declinazione::Paradigma,
}

impl NameEntry {
    pub fn italian(&self) -> &str {
        &self.italian
    }

    pub fn latin(&self) -> &declinazione::Paradigma {
        &self.latin
    }
}

#[derive(Default)]
pub struct VerbEntry {
    italian: String,
    latin: verbs::Paradigma,
}

impl VerbEntry {
    pub fn italian(&self) -> &str {
        &self.italian
    }

    pub fn latin(&self) -> &verbs::Paradigma {
        &self.latin
    }
}

#[derive(Default)]
pub struct DB {
    db_names: [Vec<NameEntry>; DeclinazioneConiugazione::__Count as usize],
    db_verbs: [Vec<VerbEntry>; DeclinazioneConiugazione::__Count as usize],
}

#[derive(Clone, Copy, Default)]
pub struct Id {
    con_dec: usize,
    idx: usize,
}

impl Id {
    fn new(con_dec: usize, idx: usize) -> Self {
        Self { con_dec, idx }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SectionCategory {
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
        match $split {
            Some(s) => s,
            None => return $error,
        }
    };
}

impl DB {
    fn parse_line<'a>(&mut self, line: &'a str) -> LineType<'a> {
        if line.starts_with('#') {
            LineType::Comment
        } else if line.is_empty() {
            LineType::Empty
        } else if line == "Names{" {
            LineType::Section(SectionCategory::Names)
        } else if line == "Verbs{" {
            LineType::Section(SectionCategory::Verbs)
        } else if line == "}" {
            LineType::CloseSection
        } else {
            LineType::Data(line)
        }
    }

    fn parse_section(&mut self, lines: &mut Lines<BufReader<File>>) -> Result<bool, DBError> {
        let mut section = SectionCategory::None;
        let mut italian = String::new();
        let mut latins = String::new();

        while let Some(Ok(untrim_line)) = lines.next() {
            let line = untrim_line.trim();
            match self.parse_line(line) {
                LineType::Section(section_category) => {
                    if section == SectionCategory::None {
                        section = section_category;
                    } else if let Err(e) = self.parse_section(lines) {
                        return Err(e);
                    }
                }
                LineType::Data(s) => match section {
                    SectionCategory::Names => {
                        let words: Vec<&str> = line.split(':').collect();
                        if words.len() != 2 {
                            return Err(DBError::InvalidLinePattern(untrim_line));
                        }
                        italian.clear();
                        latins.clear();

                        italian.push_str(words[0]);
                        latins.push_str(words[1]);

                        let mut split = latins.split(',');

                        let lat_nom = check_split!(
                            split.next(),
                            Err(DBError::InvalidLinePattern(untrim_line))
                        );
                        let lat_gen = check_split!(
                            split.next(),
                            Err(DBError::InvalidLinePattern(untrim_line))
                        );

                        let paradigma =
                            declinazione::Paradigma::new(lat_nom.to_string(), lat_gen.to_string());
                        let dec = paradigma.get_declinazione();

                        let (num, _) = match dec {
                            Ok(d) => d,
                            Err(e) => {
                                println!("error finding declinazione of {paradigma}: {e}");
                                continue;
                            }
                        };

                        self.db_names[num].push(NameEntry {
                            italian: italian.clone(),
                            latin: paradigma,
                        });
                    }
                    SectionCategory::Verbs => {
                        let words: Vec<&str> = line.split(':').collect();
                        if words.len() != 2 {
                            return Err(DBError::InvalidLinePattern(untrim_line));
                        }
                        italian.clear();
                        latins.clear();

                        italian.push_str(words[0]);
                        latins.push_str(words[1]);

                        let mut split = latins.split(',');

                        let ind_pre_1_pers = check_split!(
                            split.next(),
                            Err(DBError::InvalidLinePattern(untrim_line))
                        );
                        let ind_pre_2_pers = check_split!(
                            split.next(),
                            Err(DBError::InvalidLinePattern(untrim_line))
                        );
                        let perfetto = check_split!(
                            split.next(),
                            Err(DBError::InvalidLinePattern(untrim_line))
                        );
                        let supino = check_split!(
                            split.next(),
                            Err(DBError::InvalidLinePattern(untrim_line))
                        );
                        let infinito = check_split!(
                            split.next(),
                            Err(DBError::InvalidLinePattern(untrim_line))
                        );

                        let paradigma = verbs::Paradigma::new([
                            ind_pre_1_pers.to_string(),
                            ind_pre_2_pers.to_string(),
                            perfetto.to_string(),
                            supino.to_string(),
                            infinito.to_string(),
                        ]);
                        let con = paradigma.get_coniugazione();

                        let num = match con {
                            Ok(c) => c,
                            Err(e) => {
                                println!("error finding Coniugazione of {paradigma}: {e}");
                                continue;
                            }
                        };

                        self.db_verbs[num].push(VerbEntry {
                            italian: italian.clone(),
                            latin: paradigma,
                        });
                    }
                    SectionCategory::None => {
                        println!("WARNING: {s} is not in a category and will be lost");
                    }
                },
                LineType::Comment | LineType::Empty => (),
                LineType::CloseSection => return Ok(false),
            }
        }

        Ok(true)
    }

    pub fn init(&mut self, path: &str) -> Result<(), DBError> {
        let mut lines = match read_lines(path) {
            Ok(l) => l,
            Err(e) => {
                return Err(DBError::IO(e));
            }
        };

        loop {
            match self.parse_section(&mut lines) {
                Ok(true) => return Ok(()),
                Err(e) => return Err(e),
                _ => (),
            }
        }
    }

    pub fn get_name(&self, id: &Id) -> Option<&NameEntry> {
        if let Some(vec) = self.db_names.get(id.con_dec)
            && let Some(name) = vec.get(id.idx)
        {
            Some(name)
        } else {
            None
        }
    }

    pub fn get_verb(&self, id: &Id) -> Option<&VerbEntry> {
        if let Some(vec) = self.db_verbs.get(id.con_dec)
            && let Some(verb) = vec.get(id.idx)
        {
            Some(verb)
        } else {
            None
        }
    }

    pub fn get_rand_lat(
        &self,
        cat: SectionCategory,
        dec_con: DeclinazioneConiugazione,
    ) -> Option<(Id, &dyn GeneralPradigma)> {
        let dec_con = usize::from(dec_con);
        match cat {
            SectionCategory::Names => {
                if self.db_names.is_empty() {
                    return None;
                }
                let dec_table = &self.db_names[dec_con];
                if dec_table.is_empty() {
                    return None;
                }
                let name_idx = rng().random_range(0..dec_table.len());
                let paradigma = dec_table[name_idx].latin();
                let id = Id::new(dec_con, name_idx);

                Some((id, paradigma))
            }
            SectionCategory::Verbs => {
                if self.db_verbs.is_empty() {
                    return None;
                }
                let con_table = &self.db_verbs[dec_con];
                if con_table.is_empty() {
                    return None;
                }
                let verb_idx = rng().random_range(0..con_table.len());
                let verb_entry = &con_table[verb_idx];
                let id = Id::new(dec_con, verb_idx);

                Some((id, &verb_entry.latin))
            }
            SectionCategory::None => None,
        }
    }

        pub fn get_rand_it_with_dec_list(
            &self,
            cat: SectionCategory,
            list: &[DeclinazioneConiugazione],
            len: usize,
            ) -> Option<(Id, &str)>
        {
            let mut rng = rand::rng();
            let idx = rng.random_range(0..len);
            let dec_to_test = match list.get(idx) {
                Some(dec) => *dec,
                None => unreachable!("{idx} >= {}", list.len()),
            };
            self.get_rand_it(cat, dec_to_test)
        }

    pub fn get_rand_it(
        &self,
        cat: SectionCategory,
        dec_con: DeclinazioneConiugazione,
    ) -> Option<(Id, &str)> {
        let dec_con = usize::from(dec_con);
        match cat {
            SectionCategory::Names => {
                let names = &self.db_names;
                if names.is_empty() {
                    return None;
                }
                let dec_table = &names[dec_con];
                if dec_table.is_empty() {
                    return None;
                }
                let name_idx = rng().random_range(0..names[dec_con].len());
                let name_entry = &dec_table[name_idx];
                Some((Id::new(dec_con, name_idx), &name_entry.italian))
            }
            SectionCategory::Verbs => {
                let verbs = &self.db_verbs;
                if verbs.is_empty() {
                    return None;
                }
                let con_table = &verbs[dec_con];
                if con_table.is_empty() {
                    return None;
                }
                let verb_idx = rng().random_range(0..verbs[dec_con].len());
                let name_entry = &con_table[verb_idx];
                Some((Id::new(dec_con, verb_idx), &name_entry.italian))
            }
            SectionCategory::None => None,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::InvalidLinePattern(n) => write!(f, "invalid pattern at line {n}"),
            DBError::IO(error) => write!(f, "{error}"),
        }
    }
    // add code here
}

impl Display for SectionCategory{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SectionCategory::Names => "names",
            SectionCategory::Verbs => "verbs",
            SectionCategory::None => "none",
        })
    }
    // add code here
}

impl From<usize> for SectionCategory {
    fn from(value: usize) -> Self {
        match value {
            0 => SectionCategory::Names,
            1 => SectionCategory::Verbs,
            2 => SectionCategory::None,

            _ => unreachable!(),
        }
    }
    // add code here
}
