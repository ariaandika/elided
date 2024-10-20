#![allow(dead_code)]
use macros::dispatch;
struct Mp3;
struct Mp4;
struct Mp5;

dispatch! {
    trait Player {
        fn play(&mut self, id: usize);
    }

    enum Players {
        Mp3(Mp3),
        Mp4(Mp4),
        Mp5(Mp5),
    }
}

impl Player for Mp3 {
    fn play(&mut self, id: usize) { println!("Playing Mp3 in {id}") }
}

impl Player for Mp4 {
    fn play(&mut self, id: usize) { println!("Playing Mp4 in {id}") }
}

impl Player for Mp5 {
    fn play(&mut self, id: usize) { println!("Playing Mp5 in {id}") }
}

impl Players {
    fn get_player(id: usize) -> Self {
        match id {
            5 => Self::Mp5(Mp5),
            4 => Self::Mp4(Mp4),
            _ => Self::Mp3(Mp3),
        }
    }
}

fn main() {
    let mut mp3 = Mp3;
    play(&mut mp3, 4);

    let mut p3 = Players::get_player(3);
    let mut p4 = Players::get_player(4);
    let mut p5 = Players::get_player(5);

    play(&mut p3, 10);
    play(&mut p4, 11);
    play(&mut p5, 12);
}

fn play(player: &mut impl Player, id: usize) {
    player.play(id);
}

