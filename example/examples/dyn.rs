use example::{Mp3, Mp4, Player};

fn main() {
    let mut mp3 = Mp3;
    play_music(&mut mp3);

    let mut mp4 = Mp4;
    play_music(&mut mp4);

    let mut player = get_music_dynamic(1);
    play_music(&mut *player);

    let mut player = get_music(2);
    play_music(&mut player);
}

fn play_music(player: &mut dyn Player) {
    player.play();
}

// runtime cost
fn get_music_dynamic(id: usize) -> Box<dyn Player> {
    if id == 1 {
        Box::new(Mp3)
    } else {
        Box::new(Mp4)
    }
}

// does not compile
// fn get_music_static(id: usize) -> impl Player {
//     if id == 1 {
//         Mp3
//     } else {
//         Mp4
//     }
// }

enum Players {
    Mp3(Mp3),
    Mp4(Mp4),
}

impl Player for Players {
    fn play(&mut self) {
        match self {
            Players::Mp3(mp3) => mp3.play(),
            Players::Mp4(mp4) => mp4.play(),
        }
    }
}

fn get_music(id: usize) -> Players {
    if id == 1 {
        Players::Mp3(Mp3)
    } else {
        Players::Mp4(Mp4)
    }
}

