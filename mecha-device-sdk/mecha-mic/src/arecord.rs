use std::process::Command;

pub struct MechaAudio;

pub trait AudioInterface {
    fn record_audio(seconds: u32, filename: &str) -> ();
    fn play_audio(filename: &str) -> ();
}

impl AudioInterface for MechaAudio {
    fn record_audio(seconds: u32, filename: &str) -> () {
        let command = format!("arecord -Dplughw:0,0 -f dat -d {} {}", seconds, filename);
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
    }

    fn play_audio(filename: &str) -> () {
        let command = format!("aplay {}", filename);
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
    }
}
