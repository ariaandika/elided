use example::{Mp3, Mp4};

enum Player {
    Mp3(Mp3),
    Mp4(Mp4),
}

impl From<Mp3> for Player {
    fn from(value: Mp3) -> Self {
        Self::Mp3(value)
    }
}

impl From<Mp4> for Player {
    fn from(value: Mp4) -> Self {
        Self::Mp4(value)
    }
}

fn main() {
    let _player: Player = Mp3.into();
    let _player: Player = Mp4.into();
}

