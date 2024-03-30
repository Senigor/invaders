use rusty_audio::Audio;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    for item in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(item, &format!("audio/{}.wav", item));
    }

    // Runs in a separate thread, so wait is needed
    audio.play("startup");

    // Cleanup
    audio.wait();
    Ok(())
}
