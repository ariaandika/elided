pub struct Mp3;
pub struct Mp4;

pub trait Player {
    fn play(&mut self);
}

impl Player for Mp3 {
    fn play(&mut self) {
        println!("Playing Mp3");
    }
}
impl Player for Mp4 {
    fn play(&mut self) {
        println!("Playing Mp4");
    }
}

