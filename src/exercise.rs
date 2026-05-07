use std::fmt::Write;
use rand::RngExt;

use crate::verbs::Coniugazione;
use crate::{declinazione, verbs};
use crate::declinazione::{Declinazioni};
use crate::DB;
use std::fmt::Display;

pub const QUIT_COMMAND : &str = "QUIT";

enum DirectionTraduction{
    ItalianoLatino,
    LatinoItaliano,

    __Count
}

#[derive(Clone, Copy)]
pub enum LexicalType {
    Names,
    Verbs,
}

#[derive(Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum Declinazione{
    I,
    II,
    III,
    IV,

    __Count
}

#[derive(Clone, Copy)]
pub enum Exercise{
    Lexical((Option<[LexicalType; 2]>, usize)),
    DeclinaName((Option<[Declinazioni; 4]>, usize)),
    ConiugaVerb((Option<[Coniugazione; 4]>, usize)),

    #[allow(nonstandard_style)]
    __Count
}

enum Question{
    NameMemoryLat(usize),
    VerbMemoryLat(usize),
    NameDecLat(usize),
    VerbDecLat(usize),

    NameMemoryIt(usize),
    VerbMemoryIt(usize),
    NameDecIt(usize),
    VerbDecIt(usize),
}

#[derive(Default)]
pub struct ExerciseCheck<'a>{
    db: Option<&'a DB>,
    checkable : [Exercise; 4],
    amount_to_check: usize,
    q_type: Option<Question>,
}

impl<'a> ExerciseCheck<'a>{
    pub fn add_db(&mut self, db: &'a DB) {
        self.db = Some(db);
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        if self.amount_to_check < usize::from(Exercise::__Count) {
            self.checkable[self.amount_to_check] = exercise;
            self.amount_to_check+=1;
        }
    }

    pub fn num_exercise (&self) -> usize {
        self.amount_to_check
    }

    pub fn question(&mut self, buffer: &mut String){
        use rand;
        let mut rng = rand::rng();
        let q_type = rng.random_range(0..self.amount_to_check);
        let mut question = None;
        let dir_trad =DirectionTraduction::from(rng.random_range(0..DirectionTraduction::__Count as usize)); 
        let con_dec_to_ask;
        buffer.clear();

        match self.checkable[q_type]{
            Exercise::Lexical((list, len)) => {
                let l_type = list.unwrap()[rng.random_range(0..len)];
                con_dec_to_ask = {
                    let idx = rng.random_range(0..len);
                    usize::from(list.unwrap()[idx])
                };
                let _ = write!(buffer, "traduci ");
                match dir_trad {
                    DirectionTraduction::ItalianoLatino => {
                        let _ = write!(buffer, "dall'italiano al latino ");
                    },
                    DirectionTraduction::LatinoItaliano => {
                        let _ = write!(buffer, "dal latino all'italiano ");
                    },
                    DirectionTraduction::__Count => unreachable!(),
                }
                if let Some(db) = &self.db{
                    match l_type{
                        LexicalType::Names => {
                            let dec = declinazione::Declinazioni::from(con_dec_to_ask);
                            match dir_trad {
                                DirectionTraduction::ItalianoLatino => {
                                    let (idx, name) = db.get_rand_name_it(dec);
                                    let _ = write!(buffer, "{}", name);
                                    question = Some(Question::NameMemoryIt(idx))
                                },
                                DirectionTraduction::LatinoItaliano => {
                                    let (idx, paradigma) = db.get_rand_name_lat(dec);
                                    let _ = write!(buffer, "{}", paradigma);
                                    question = Some(Question::NameMemoryLat(idx))
                                },
                                DirectionTraduction::__Count => unreachable!(),
                            }
                        }
                        LexicalType::Verbs => {
                            let con = verbs::Coniugazione::from(con_dec_to_ask);
                            match dir_trad {
                                DirectionTraduction::ItalianoLatino => {
                                    let (idx, verb) = db.get_rand_verb_it(con);
                                    let _ = write!(buffer, "{}", verb);
                                    question = Some(Question::VerbMemoryIt(idx))
                                },
                                DirectionTraduction::LatinoItaliano => {
                                    let (idx, paradigma) = db.get_rand_verb_lat(con);
                                    let _ = write!(buffer, "{}", paradigma);
                                    question = Some(Question::VerbMemoryLat(idx))
                                },
                                DirectionTraduction::__Count => unreachable!(),
                            }
                        },
                    }

                    let _ = write!(buffer, ": ");
                }
                else{
                    println!("no db")
                }
            },
            Exercise::DeclinaName(_) => todo!(),
            Exercise::ConiugaVerb(_) => todo!(),
            Exercise::__Count => unreachable!(),
        }

        self.q_type = question;
    }

    pub fn answer(&self, answer: &str) -> bool{
        fn missing_input() -> bool{
            println!("missing input");
            let _ =std::io::Write::flush(&mut ::std::io::stdout());
            false
        }

        fn good_job() -> bool{
            println!("good job");
            let _ =std::io::Write::flush(&mut ::std::io::stdout());
            true
        }

        fn incorrect_answer(given: &str, expected: &str) -> bool {
            println!("error: given {}, expected: {}", given, expected);
            let _ =std::io::Write::flush(&mut ::std::io::stdout());
            false
        }

        if let Some(db) = self.db && let Some(question) = &self.q_type{
            match question {
                Question::NameMemoryLat(id) => {
                    let correct_it = db.get_name(*id);
                    match correct_it{
                        Some(correct_it) => {
                            match correct_it.italian == answer{
                                true => good_job(),
                                false => {
                                    incorrect_answer(answer, &correct_it.italian)
                                },
                            }
                        },
                        None => false,
                    }
                },
                Question::VerbMemoryLat(id) => {
                    let correct_it = db.get_verb(*id);
                    match correct_it{
                        Some(correct_it) => {
                            match correct_it.italian == answer{
                                true => good_job(),
                                false => incorrect_answer(answer, &correct_it.italian),
                            }
                        },
                        None => false,
                    }
                },
                Question::NameDecLat(_) => false,
                Question::VerbDecLat(_) => false,
                Question::NameMemoryIt(id) => {
                    if let Some(name) = db.get_name(*id){
                        let mut split = answer.split(",");
                        let nominativo = split.next();
                        let genitivo = split.next();

                        match (nominativo,genitivo){
                            (Some(nominativo), Some(genitivo)) => {
                                match nominativo == name.latin[0] && genitivo == name.latin[1]{
                                    true => good_job(),
                                    false => incorrect_answer(
                                        &format!("{},{}", nominativo, genitivo), 
                                        &format!("{},{}", name.latin[0], name.latin[1]))
                                        ,
                                }
                            },
                            _ => missing_input(),
                        }
                    }
                    else{
                        false
                    }
                },
                Question::VerbMemoryIt(id) => {
                    if let Some(verb) = db.get_verb(*id){
                        let mut split = answer.split(",");
                        let mut parad = ["","","","",""];
                        let verb = &verb.latin;

                        for cell in &mut parad{
                            *cell = match split.next(){
                                Some(s) => s,
                                None => {
                                    missing_input();
                                    return false;
                                },
                            }
                        }

                        for i in 0..parad.len(){
                            if verb[i] != parad[i]{
                                return incorrect_answer(
                                    &format!("{},{},{},{},{}",
                                        parad[0], parad[1], parad[2], parad[3], parad[4]), 
                                    &format!("{},{},{},{},{}",
                                        verb[0], verb[1], verb[2], verb[3], verb[4]));
                            }
                        }

                        good_job()
                    }
                    else{
                        false
                    }
                },
                Question::NameDecIt(_) => false,
                Question::VerbDecIt(_) => false,
            }
        }
        else
        {
            false
        }
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct ExeRes{
    pub sucess: usize,
    pub failure: usize
}

#[allow(dead_code)]
impl ExeRes {
    pub fn success(&mut self){
        self.sucess+=1;
    }
    pub fn fail(&mut self){
        self.failure+=1;
    }
}

impl From<Exercise> for usize{
    fn from(value: Exercise) -> Self {
        match value {
            Exercise::Lexical(_) => 0,
            Exercise::DeclinaName(_) => 1,
            Exercise::ConiugaVerb(_) => 2,
            Exercise::__Count => 3,
        }
    }
    // add code here
}

impl From<Declinazione> for usize{
    fn from(value: Declinazione) -> Self {
        value as Self 
    }
    // add code here
}

impl From<LexicalType> for usize{
    fn from(value: LexicalType) -> Self {
        value as Self 
    }
    // add code here
}

impl From<DirectionTraduction> for usize{
    fn from(value: DirectionTraduction) -> Self {
        value as Self 
    }
    // add code here
}

impl From<usize> for LexicalType{
    fn from(value: usize) -> Self {
        match value {
            0 => LexicalType::Names,
            1 => LexicalType::Verbs,
            _ => unreachable!(),
            
        }
    }
    // add code here
}

impl From<usize> for DirectionTraduction{
    fn from(value: usize) -> Self {
        match value {
            0 => DirectionTraduction::ItalianoLatino,
            1 => DirectionTraduction::LatinoItaliano,
            _ => unreachable!(),
            
        }
    }
    // add code here
}

impl Display for ExeRes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "success: {}, failure: {}", self.sucess, self.failure)
    }
}

impl Default for Exercise{
    fn default() -> Self {
        Self::Lexical((None, 0))
    }
    // add code here
}
