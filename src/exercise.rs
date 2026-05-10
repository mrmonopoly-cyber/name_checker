use rand::RngExt;
use std::fmt::Display;
use std::fmt::Write;

use crate::DB;
use crate::common::{DeclinazioneConiugazione, Numero};
use crate::db::{Id, SectionCategory};
use crate::declinazione::Casi;
use crate::verbs::{Modo, Persona, Tempo};

pub const QUIT_COMMAND: &str = "QUIT";


enum DirectionTraduction {
    ItalianoLatino,
    LatinoItaliano,

    __Count,
}

pub enum QuestionError {
    NoDB,
    NoData(SectionCategory),
    InvalidExercise,
}

#[derive(Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum Declinazione {
    I,
    II,
    III,
    IV,

    __Count,
}

#[derive(Clone, Copy)]
pub enum Exercise {
    Lexical((Option<[SectionCategory; 2]>, usize)),
    DeclinaName((Option<[DeclinazioneConiugazione; 4]>, usize)),
    ConiugaVerb((Option<[DeclinazioneConiugazione; 4]>, usize)),

    __Count,
}

enum Question {
    LexicalLat(SectionCategory, Id),
    LexicalIt(SectionCategory, Id),

    NameDecLat(Id, Casi, Numero),
    VerbDecLat(Id, Modo, Tempo, Persona, Numero),
}

#[derive(Default)]
pub struct ExerciseCheck<'a> {
    db: Option<&'a DB>,
    checkable: [Exercise; 4],
    amount_to_check: usize,
    q_type: Option<Question>,
}

impl<'a> ExerciseCheck<'a> {
    pub fn add_db(&mut self, db: &'a DB) {
        self.db = Some(db);
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        if self.amount_to_check < usize::from(Exercise::__Count) {
            self.checkable[self.amount_to_check] = exercise;
            self.amount_to_check += 1;
        }
    }

    pub fn num_exercise(&self) -> usize {
        self.amount_to_check
    }

    pub fn question(&mut self, buffer: &mut String) -> Result<(), QuestionError>{

        fn get_verb_name_dec_con<'a>(
            len: usize,
            list: &'a[DeclinazioneConiugazione],
            db: &'a DB,
            cat: SectionCategory) -> Result<(Id, &'a str), QuestionError>
        {
            match db.get_rand_it_with_dec_list(cat, list, len){
                Some(pair) => Ok(pair),
                None => Err(QuestionError::NoData(cat)),
            }
        }

        let mut rng = rand::rng();
        let q_type = rng.random_range(0..self.amount_to_check);
        let question;
        let dir_trad =
            DirectionTraduction::from(rng.random_range(0..DirectionTraduction::__Count as usize));
        buffer.clear();

        let db = match self.db {
            Some(db) => db,
            None => {
                return Err(QuestionError::NoDB);
            }
        };

        match self.checkable[q_type] {
            Exercise::Lexical((list, len)) => {
                let l_type = list.unwrap()[rng.random_range(0..len)];
                let _ = write!(buffer, "traduci {dir_trad}");
                match dir_trad {
                    DirectionTraduction::ItalianoLatino => {
                        let (idx, it);
                            loop {
                                let dec_con = DeclinazioneConiugazione::from(rng.random_range(1..=4));
                                if let Some(name) = db.get_rand_it(l_type, dec_con) {
                                    (idx, it) = name;
                                    break;
                                }
                            }

                        let _ = write!(buffer, "{}", it);
                        question = Some(Question::LexicalIt(l_type, idx))
                    }
                    DirectionTraduction::LatinoItaliano => {
                        let (idx, paradigma);
                        loop {
                            let dec_con = DeclinazioneConiugazione::from(rng.random_range(1..=4));
                            if let Some(name) = db.get_rand_lat(l_type, dec_con) {
                                (idx, paradigma) = name;
                                break;
                            }
                        }

                        let _ = write!(buffer, "{}", paradigma);
                        question = Some(Question::LexicalLat(l_type, idx))
                    }
                    DirectionTraduction::__Count => unreachable!(),
                }
                let _ = write!(buffer, ": ");
            }
            Exercise::DeclinaName((Some(list), len)) => {
                let (idx, name) = get_verb_name_dec_con(len, &list, db, SectionCategory::Names)?;
                let caso = Casi::from(rng.random_range(0..usize::from(Casi::__Count)));
                let numero = Numero::from(rng.random_range(0..usize::from(Numero::__Num__Numero)));

                let _ = write!(buffer, "dimmi il {caso} {numero} di {name}: ");
                question = Some(Question::NameDecLat(idx, caso, numero));
            }
            Exercise::ConiugaVerb((Some(list), len)) => {
                let (idx, verb) = get_verb_name_dec_con(len, &list, db, SectionCategory::Verbs)?;
                let modo = Modo::from(rng.random_range(0..usize::from(Modo::__Modo_count)));
                let tempo = Tempo::from(rng.random_range(0..usize::from(Tempo::__Count)));
                let persona = Persona::from(rng.random_range(0..usize::from(Persona::__Count)));
                let numero = Numero::from(rng.random_range(0..usize::from(Numero::__Num__Numero)));


                let _ = write!(
                    buffer,
                    "dimmi il {modo} {tempo} {persona} {numero} di {verb}: "
                );

                question = Some(Question::VerbDecLat(idx, modo, tempo, persona, numero));
            }
            Exercise::__Count => unreachable!(),
            _ => return Err(QuestionError::InvalidExercise),
        }

        self.q_type = question;

        Ok(())
    }

    pub fn answer(&self, answer: &str) -> bool {
        fn missing_input() -> bool {
            println!("missing input");
            let _ = std::io::Write::flush(&mut ::std::io::stdout());
            false
        }

        fn good_job() -> bool {
            println!("good job");
            let _ = std::io::Write::flush(&mut ::std::io::stdout());
            true
        }

        fn incorrect_answer(given: &str, expected: &str) -> bool {
            println!("error: given {}, expected: {}", given, expected);
            let _ = std::io::Write::flush(&mut ::std::io::stdout());
            false
        }

        if let Some(db) = self.db
            && let Some(question) = &self.q_type
        {
            match question {
                Question::LexicalLat(cat, id) => {
                    let correct_it = match cat {
                        SectionCategory::Names => match db.get_name(id) {
                            Some(e) => e.italian(),
                            None => return false,
                        },
                        SectionCategory::Verbs => match db.get_verb(id) {
                            Some(e) => e.italian(),
                            None => return false,
                        },
                        SectionCategory::None => unreachable!(),
                    };

                    correct_it == answer
                }

                Question::LexicalIt(cat, id) => {
                    let mut split = answer.split(",");
                    match cat {
                        SectionCategory::Names => {
                            if let Some(name) = db.get_name(id) {
                                let nominativo = split.next();
                                let genitivo = split.next();
                                let latin = name.latin();

                                match (nominativo, genitivo) {
                                    (Some(nominativo), Some(genitivo)) => {
                                        match nominativo == latin.nominativo()
                                            && genitivo == latin.genitivo()
                                        {
                                            true => good_job(),
                                            false => incorrect_answer(
                                                &format!("{},{}", nominativo, genitivo),
                                                &format!(
                                                    "{},{}",
                                                    latin.nominativo(),
                                                    latin.genitivo()
                                                ),
                                            ),
                                        }
                                    }
                                    _ => missing_input(),
                                }
                            } else {
                                false
                            }
                        }
                        SectionCategory::Verbs => {
                            let mut parad = ["", "", "", "", ""];
                            for cell in &mut parad {
                                *cell = match split.next() {
                                    Some(s) => s,
                                    None => {
                                        missing_input();
                                        return false;
                                    }
                                }
                            }

                            if let Some(verb) = db.get_verb(id) {
                                let latin = verb.latin().verb_list();

                                for i in 0..parad.len() {
                                    if latin[i] != parad[i] {
                                        incorrect_answer(
                                            &format!(
                                                "{},{},{},{},{}",
                                                parad[0], parad[1], parad[2], parad[3], parad[4]
                                            ),
                                            &format!(
                                                "{},{},{},{},{}",
                                                latin[0], latin[1], latin[2], latin[3], latin[4]
                                            ),
                                        );
                                    }
                                }

                                good_job()
                            } else {
                                false
                            }
                        }
                        SectionCategory::None => unreachable!(),
                    }
                }
                Question::NameDecLat(name_id, caso, numero) => {
                    let name = match db.get_name(name_id) {
                        Some(name) => name.latin(),
                        None => {
                            println!("invalid idx name in answer");
                            return false;
                        }
                    };

                    let declinato = match name.declina(*caso, *numero) {
                        Ok(dec) => dec,
                        Err(e) => {
                            println!("error declinazione di {name}: {e}");
                            return false;
                        }
                    };
                    match declinato == answer {
                        true => good_job(),
                        false => incorrect_answer(answer, &declinato),
                    }
                }
                Question::VerbDecLat(verb_id, modo, tempo, persona, numero) => {
                    let verb = match db.get_verb(verb_id) {
                        Some(verb) => verb.latin(),
                        None => {
                            println!("invalid idx verb in answer");
                            return false;
                        }
                    };

                    let coniugato = match verb.coniuga_verbo(*modo, *tempo, *persona, *numero) {
                        Ok(dec) => dec,
                        Err(e) => {
                            println!("error coniugazione di {verb}: {e}");
                            return false;
                        }
                    };

                    match coniugato == answer {
                        true => good_job(),
                        false => incorrect_answer(answer, &coniugato),
                    }
                }
            }
        } else {
            false
        }
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct ExeRes {
    pub sucess: usize,
    pub failure: usize,
}

#[allow(dead_code)]
impl ExeRes {
    pub fn success(&mut self) {
        self.sucess += 1;
    }
    pub fn fail(&mut self) {
        self.failure += 1;
    }
}

impl From<Exercise> for usize {
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

impl From<Declinazione> for usize {
    fn from(value: Declinazione) -> Self {
        value as Self
    }
    // add code here
}

impl From<DirectionTraduction> for usize {
    fn from(value: DirectionTraduction) -> Self {
        value as Self
    }
    // add code here
}

impl From<usize> for DirectionTraduction {
    fn from(value: usize) -> Self {
        match value {
            0 => DirectionTraduction::ItalianoLatino,
            1 => DirectionTraduction::LatinoItaliano,
            _ => unreachable!(),
        }
    }
    // add code here
}

impl Display for ExeRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "success: {}, failure: {}", self.sucess, self.failure)
    }
}

impl Display for DirectionTraduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionTraduction::ItalianoLatino => {
                write!(f, "dall'italiano al latino ")
            }
            DirectionTraduction::LatinoItaliano => {
                write!(f, "dal latino all'italiano ")
            }
            DirectionTraduction::__Count => unreachable!(),
        }
    }
    // add code here
}

impl Display for QuestionError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionError::NoDB => write!(f, "no db"),
            QuestionError::NoData(section_category) => write!(f, "no data for {section_category}"),
            QuestionError::InvalidExercise => write!(f, "invalid exercise"),
        }
    }
    // add code here
}

impl Default for Exercise {
    fn default() -> Self {
        Self::Lexical((None, 0))
    }
    // add code here
}
