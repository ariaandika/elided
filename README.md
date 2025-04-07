# Rust Pattern

common pattern that can be automated using macro

## Enum `From` Variants

a pattern where enum implement `From` for each variants

see example in `example/examples/enum-from.rs`

```rust
struct Mp3;
struct Mp4;

enum Player {
    Mp3(Mp3),
    Mp4(Mp4),
}

impl From<Mp3> for Player { /* */ }
impl From<Mp4> for Player { /* */ }

fn main() {
    let player: Player = Mp3.into();
    let player: Player = Mp4.into();
}
```

we can automate the `From` implementation

```rust
use macros::FromUnit;

struct Mp3;
struct Mp4;

#[derive(FromUnit)]
enum Player {
    Mp3(Mp3),
    Mp4(Mp4),
}

fn main() {
    let player: Player = Mp3.into();
    let player: Player = Mp4.into();
}
```

## Manual Dispatch

polymorphism is rust uses either generics via static dispatch,
or trait object via dynamic dispatch

see example in `example/examples/manual-dispatch.rs`

```rust
struct Mp3;
struct Mp4;

trait Player {
    fn play(&mut self);
}

impl Player for Mp3 { /* .. */ }
impl Player for Mp4 { /* .. */ }

fn main() {
    let mut mp3 = Mp3;
    play_music(&mut mp3);
    let mut mp4 = Mp4;
    play_music(&mut mp4);
}

fn play_music(player: &mut dyn Player) {
    player.play();
}
```

but there is a catch

```rust
// runtime cost
fn get_music_dynamic(id: usize) -> Box<dyn Player> {
    if id == 1 { Box::new(Mp3) } else { Box::new(Mp4) }
}

// does not compile
fn get_music_static(id: usize) -> impl Player {
    if id == 1 { Mp3 } else { Mp4 }
}
```

we can use manual dispatch (its not official name, i just made it up)

```rust
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

fn main() {
    let mut player = get_music(1);
    play_music(&mut player);
}

fn play_music(player: &mut impl Player) {
    player.play();
}
```

we can automate the trait implementation for the enum

```rust
macros::dispatch! {
    enum PlayersV3 {
        Mp3(Mp3)
        Mp4(Mp4)
    }
    impl Player for PlayerV3 {
        fn play(&mut self, id: usize) { }
    }
}

fn get_music(id: usize) -> Players {
    if id == 1 {
        Players::Mp3(Mp3)
    } else {
        Players::Mp4(Mp4)
    }
}

fn main() {
    let mut player = get_music(1);
    play_music(&mut player);
}

fn play_music(player: &mut impl Player) {
    player.play();
}
```

## Error Wrapper

we usually create error enum to wrap error from underlying api

```rust
enum PlayerError {
    Mp3(Mp3Error),
    Mp4(Mp4Error),
}

fn main() -> Result<(), PlayerError> {
    if let Err(err) = play_mp3() {
        return Err(PlayerError::Mp3(err))
    }
    if let Err(err) = play_mp4() {
        return Err(PlayerError::Mp4(err))
    }
    Ok(())
}
```

we can automate the `From` and `Error` implementation

```rust
#[derive(StdError)]
enum PlayerError {
    Mp3(Mp3Error),
    Mp4(Mp4Error),
}

fn main() -> Result<(), PlayerError> {
    play_mp3()?;
    play_mp4()?;
    Ok(())
}
```

