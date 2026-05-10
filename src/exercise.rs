use std::fmt::Write;
use rand::RngExt;
use std::fmt::Display;

use crate::db::Id;
use crate::verbs::{Modo, Persona, Tempo};
use crate::verbs;
use crate::declinazione::{Casi, Paradigma};
use crate::DB;
use crate::common::{DeclinazioneConiugazione, Numero};

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
    DeclinaName((Option<[DeclinazioneConiugazione; 4]>, usize)),
    ConiugaVerb((Option<[DeclinazioneConiugazione; 4]>, usize)),

    __Count
}

enum Question{
    NameMemoryLat(Id),
    VerbMemoryLat(Id),
    NameDecLat(Id, Casi, Numero),
    VerbDecLat(Id, Modo, Tempo, Persona, Numero),

    NameMemoryIt(Id),
    VerbMemoryIt(Id),
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
        let question;
        let dir_trad =DirectionTraduction::from(rng.random_range(0..DirectionTraduction::__Count as usize)); 
        let con_dec_to_ask;
        buffer.clear();

        let db = match self.db{
            Some(db) => db,
            None => {
                println!("no db");
                return;
            },
        };

        match self.checkable[q_type]{
            Exercise::Lexical((list, len)) => {
                let l_type = list.unwrap()[rng.random_range(0..len)];
                con_dec_to_ask = DeclinazioneConiugazione::from(rng.random_range(1..=4));
                let _ = write!(buffer, "traduci {dir_trad}");
                match l_type{
                    LexicalType::Names => {
                        match dir_trad {
                            DirectionTraduction::ItalianoLatino => {
                                let (idx, name) = db.get_rand_name_it(con_dec_to_ask);
                                let _ = write!(buffer, "{}", name);
                                question = Some(Question::NameMemoryIt(idx))
                            },
                            DirectionTraduction::LatinoItaliano => {
                                let (idx, paradigma) = db.get_rand_name_lat(con_dec_to_ask);
                                let _ = write!(buffer, "{}", paradigma);
                                question = Some(Question::NameMemoryLat(idx))
                            },
                            DirectionTraduction::__Count => unreachable!(),
                        }
                    }
                    LexicalType::Verbs => {
                        match dir_trad {
                            DirectionTraduction::ItalianoLatino => {
                                let (idx, verb) = db.get_rand_verb_it(con_dec_to_ask);
                                let _ = write!(buffer, "{}", verb);
                                question = Some(Question::VerbMemoryIt(idx))
                            },
                            DirectionTraduction::LatinoItaliano => {
                                let (idx, paradigma) = db.get_rand_verb_lat(con_dec_to_ask);
                                let _ = write!(buffer, "{}", paradigma);
                                question = Some(Question::VerbMemoryLat(idx))
                            },
                            DirectionTraduction::__Count => unreachable!(),
                        }
                    },
                }

                let _ = write!(buffer, ": ");
            },
            Exercise::DeclinaName((Some(list), len)) => {
                let idx = rng.random_range(0..len);
                let dec_to_test = match list.get(idx){
                    Some(dec) => *dec,
                    None => unreachable!("{idx} >= {len}"),
                };
                let caso = Casi::from(rng.random_range(0..usize::from(Casi::__Count)));
                let numero = Numero::from(rng.random_range(0..usize::from(Numero::__Num__Numero)));
                let (idx,name) = db.get_rand_name_lat(dec_to_test);

                let _ = write!(buffer, "dimmi il {caso} {numero} di {name}: ");

                question = Some(Question::NameDecLat(idx, caso, numero));
            },
            Exercise::ConiugaVerb((Some(list), len)) => {
                let idx = rng.random_range(0..len);
                let dec_to_test = match list.get(idx){
                    Some(dec) => *dec,
                    None => unreachable!("{idx} >= {len}"),
                };
                let modo = Modo::from(rng.random_range(0..usize::from(Modo::__Modo_count)));
                let tempo = Tempo::from(rng.random_range(0..usize::from(Tempo::__Count)));
                let persona = Persona::from(rng.random_range(0..usize::from(Persona::__Count)));
                let numero = Numero::from(rng.random_range(0..usize::from(Numero::__Num__Numero)));
                let (idx,verb) = db.get_rand_verb_lat(dec_to_test);

                let _ = write!(buffer, "dimmi il {modo} {tempo} {persona} {numero} di {verb}: ");

                question = Some(Question::VerbDecLat(idx, modo, tempo, persona, numero));
            },
            Exercise::__Count => unreachable!(),
            _ => return,
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
                Question::NameDecLat(name_id, caso, numero) => {
                    let name = match db.get_name(*name_id){
                        Some(name) => Paradigma::new(&name.latin[0], &name.latin[1]),
                        None => {
                            println!("invalid idx name in answer");
                            return false;
                        },
                    };

                    let declinato = match name.declina(*caso, *numero){
                        Ok(dec) => dec,
                        Err(e) => {
                            println!("error declinazione di {name}: {e}");
                            return false;
                        },
                    };
                    match declinato == answer{
                        true => good_job(),
                        false => incorrect_answer(answer, &declinato),
                    }
                },
                Question::VerbDecLat(verb_id, modo, tempo, persona, numero) => {
                    let verb = match db.get_verb(*verb_id){
                        Some(verb) => verbs::Paradigma::new(&verb.latin),
                        None => {
                            println!("invalid idx verb in answer");
                            return false;
                        },
                    };

                    let coniugato = match verb.coniuga_verbo(*modo, *tempo, *persona, *numero){
                        Ok(dec) => dec,
                        Err(e) => {
                            println!("error coniugazione di {verb}: {e}");
                            return false;
                        },
                    };
                    
                    match coniugato == answer{
                        true => good_job(),
                        false => incorrect_answer(answer, &coniugato),
                    }
                },
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

impl Display for DirectionTraduction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            DirectionTraduction::ItalianoLatino => {
                write!(f, "dall'italiano al latino ")
            },
            DirectionTraduction::LatinoItaliano => {
                write!(f, "dal latino all'italiano ")
            },
            DirectionTraduction::__Count => unreachable!(),
        }
    }
    // add code here
}

impl Default for Exercise{
    fn default() -> Self {
        Self::Lexical((None, 0))
    }
    // add code here
}
