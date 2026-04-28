use std::fmt::Display;

pub const QUIT_COMMAND : &str = "QUIT";

#[derive(Default)]
pub struct ExeRes{
    pub sucess: usize,
    pub failure: usize
}

impl ExeRes {
    pub fn success(&mut self){
        self.sucess+=1;
    }
    pub fn fail(&mut self){
        self.failure+=1;
    }
}

impl Display for ExeRes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "success: {}, failure: {}", self.sucess, self.failure)
    }
}


pub trait Exercise {

    fn run_exercise(&self) -> ExeRes;
}
