#![allow(dead_code)]
use macros::FromUnit;

struct Mp3;
struct Mp4;
struct Mp5;

#[derive(FromUnit)]
enum Player {
    Mpg,
    Mp3(Mp3),
    Mp4(Mp4),
    Mpgs(Mp3,Mp4),
    Mp5 {
        mp5: Mp5,
    },
    Mps {
        mp3: Mp3,
        mp4: Mp4,
        mp5: Mp5,
    }
}

fn main() {
    let _player: Player = Mp3.into();
    let _player: Player = (Mp3,Mp4,Mp5).into();
}

