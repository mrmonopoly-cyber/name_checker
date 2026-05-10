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
    NoData(Exercise),
}

#[derive(Clone, Copy)]
pub struct ConDecListToTest {
    pool: [DeclinazioneConiugazione; 4],
    len: usize,
}

impl ConDecListToTest {
    pub fn add_dec_con(&mut self, dec_con: DeclinazioneConiugazione) {
        for idx in 0..self.len {
            if self.pool[idx] == dec_con {
                return;
            }
        }

        self.pool[self.len] = dec_con;
        self.len += 1;
    }

    pub fn is_active(&self) -> bool {
        self.len > 0
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.pool = [DeclinazioneConiugazione::I;4];
    }
}

#[derive(Clone, Copy)]
pub enum ExerciseType {
    LexicalName,
    LexicalVerb,
    DeclinaName,
    ConiugaVerb,

    __Count,
}

#[derive(Clone, Copy)]
pub struct Exercise {
    t: ExerciseType,
    decs: ConDecListToTest,
}

impl Exercise {
    pub fn new(t: ExerciseType, decs: ConDecListToTest) -> Self {
        Self { t, decs }
    }
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
        if self.amount_to_check < usize::from(ExerciseType::__Count) {
            self.checkable[self.amount_to_check] = exercise;
            self.amount_to_check += 1;
        }
    }

    pub fn num_exercise(&self) -> usize {
        self.amount_to_check
    }

    fn lexical_question(
        &mut self,
        decs: &ConDecListToTest,
        buffer: &mut String,
        cat: SectionCategory,
        dir_trad: DirectionTraduction,
    ) -> Option<Question> {
        let mut rng = rand::rng();
        let l_type = decs.pool[rng.random_range(0..decs.len)];
        let db = self.db.unwrap();

        let _ = write!(buffer, "traduci {dir_trad}");

        match dir_trad {
            DirectionTraduction::ItalianoLatino => {
                let (idx, it) = db.get_rand_it(cat, l_type)?;
                let _ = write!(buffer, "{}: ", it);
                Some(Question::LexicalIt(SectionCategory::Names, idx))
            }
            DirectionTraduction::LatinoItaliano => {
                let (idx, paradigma) = db.get_rand_lat(cat, l_type)?;
                let _ = write!(buffer, "{}: ", paradigma);
                Some(Question::LexicalLat(cat, idx))
            }
            DirectionTraduction::__Count => unreachable!(),
        }
    }

    pub fn question(&mut self, buffer: &mut String) -> Result<(), QuestionError> {
        let mut rng = rand::rng();
        let q_type = rng.random_range(0..self.amount_to_check);
        let dir_trad =
            DirectionTraduction::from(rng.random_range(0..DirectionTraduction::__Count as usize));
        buffer.clear();

        let db = match self.db {
            Some(db) => db,
            None => {
                return Err(QuestionError::NoDB);
            }
        };

        let exer = self.checkable[q_type];
        let decs = &exer.decs;

        if !exer.decs.is_active() {
            return Err(QuestionError::NoData(exer));
        }

        self.q_type = match exer.t {
            ExerciseType::LexicalName => {
                match self.lexical_question(decs, buffer, SectionCategory::Names, dir_trad){
                    Some(q) => Some(q),
                    None => return Err(QuestionError::NoData(exer)),
                }
            }
            ExerciseType::LexicalVerb => {
                match self.lexical_question(decs, buffer, SectionCategory::Names, dir_trad){
                    Some(q) => Some(q),
                    None => return Err(QuestionError::NoData(exer)),
                }
            }
            ExerciseType::DeclinaName => {
                let len = decs.len;
                let list = &decs.pool;
                let cat = SectionCategory::Names;

                let (idx, name) = match db.get_rand_it_with_dec_list(cat, list, len) {
                    Some(pair) => pair,
                    None => return Err(QuestionError::NoData(exer)),
                };
                let caso = Casi::from(rng.random_range(0..usize::from(Casi::__Count)));
                let numero = Numero::from(rng.random_range(0..usize::from(Numero::__Num__Numero)));

                let _ = write!(buffer, "dimmi il {caso} {numero} di {name}: ");
                Some(Question::NameDecLat(idx, caso, numero))
            }
            ExerciseType::ConiugaVerb => {
                let len = decs.len;
                let list = &decs.pool;
                let cat = SectionCategory::Verbs;

                let (idx, verb) = match db.get_rand_it_with_dec_list(cat, list, len) {
                    Some(pair) => pair,
                    None => return Err(QuestionError::NoData(exer)),
                };
                let modo = Modo::from(rng.random_range(0..usize::from(Modo::__Modo_count)));
                let tempo = Tempo::from(rng.random_range(0..usize::from(Tempo::__Count)));
                let persona = Persona::from(rng.random_range(0..usize::from(Persona::__Count)));
                let numero = Numero::from(rng.random_range(0..usize::from(Numero::__Num__Numero)));

                let _ = write!(buffer,"dimmi il {modo} {tempo} {persona} {numero} di {verb}: ");

                Some(Question::VerbDecLat(idx, modo, tempo, persona, numero))
            }
            ExerciseType::__Count => unreachable!(),
        };

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
                                let name_nominativo = latin.nominativo();
                                let name_genitivo = latin.genitivo();

                                match (nominativo, genitivo) {
                                    (Some(nominativo), Some(genitivo)) => 
                                    {
                                        match nominativo == name_nominativo
                                            && genitivo == name_genitivo
                                        {
                                            true => good_job(),
                                            false => incorrect_answer(
                                                &format!("{},{}", nominativo, genitivo),
                                                &format!("{},{}", name_nominativo, name_genitivo),
                                            ),
                                        }
                                    },
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
                            }
                            else {
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

impl From<ExerciseType> for usize {
    fn from(value: ExerciseType) -> Self {
        match value {
            ExerciseType::LexicalName => 0,
            ExerciseType::LexicalVerb => 1,
            ExerciseType::DeclinaName => 2,
            ExerciseType::ConiugaVerb => 3,
            ExerciseType::__Count => 4,
        }
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

impl Display for ExerciseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExerciseType::LexicalName => "lexical name",
                ExerciseType::LexicalVerb => "lexical verb",
                ExerciseType::DeclinaName => "declina name",
                ExerciseType::ConiugaVerb => "coniuga verb",
                ExerciseType::__Count => todo!(),
            }
        )
    }
    // add code here
}

impl Display for QuestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionError::NoDB => write!(f, "no db"),
            QuestionError::NoData(exer) => write!(f, "no data for {}", exer.t),
        }
    }
    // add code here
}

impl Default for ConDecListToTest {
    fn default() -> Self {
        Self {
            pool: [DeclinazioneConiugazione::I; 4],
            len: 0,
        }
    }
    // add code here
}

impl Default for Exercise {
    fn default() -> Self {
        Self {
            t: ExerciseType::LexicalName,
            decs: ConDecListToTest::default(),
        }
    }
    // add code here
}
