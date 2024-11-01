#![allow(dead_code)]
use macros::StdError;

#[derive(StdError)]
enum PlayerError {
    Mp3(player::Mp3Error),
    Mp4(player::Mp4Error),
}

fn main() -> Result<(), PlayerError> {
    play_mp3()?;
    play_mp4()?;
    let _ = <PlayerError as std::error::Error>::source;
    Ok(())
}

fn play_mp3() -> Result<(), player::Mp3Error> { Ok(()) }
fn play_mp4() -> Result<(), player::Mp3Error> { Ok(()) }




mod player {
    #[derive(Debug)]
    pub struct Mp3Error;

    #[derive(Debug)]
    pub struct Mp4Error;

    impl std::error::Error for Mp3Error { }
    impl std::fmt::Display for Mp3Error {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    impl std::error::Error for Mp4Error { }
    impl std::fmt::Display for Mp4Error {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}

