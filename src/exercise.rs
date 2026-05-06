use std::fmt::Display;

pub const QUIT_COMMAND : &str = "QUIT";

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

impl Display for ExeRes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "success: {}, failure: {}", self.sucess, self.failure)
    }
}
