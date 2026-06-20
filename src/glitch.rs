use rand::prelude::SliceRandom;
use std::thread;
use std::time::Duration;
use rand::Rng;

pub fn boot() {
    let lines = vec![
        "VØSK TERMINAL v0.1.0",
        "initializing...",
        "loading filesystem...",
        "establishing context...",
        "WARNING: session integrity unknown",
        "WARNING: last shutdown unclean",
        "",
        "type 'help' for available commands",
        "",
    ];

    for line in lines {
        maybe_glitch_line(line);
        thread::sleep(Duration::from_millis(120));
    }
}

pub fn maybe_glitch_line(line: &str) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.15) {
        println!("{}", corrupt_string(line, &mut rng));
        thread::sleep(Duration::from_millis(60));
        print!("\x1B[1A\x1B[2K");
    }
    println!("{}", line);
}

pub fn corrupt_string(s: &str, rng: &mut impl Rng) -> String {
    let noise = vec!['▓', '░', '▒', '╳', '■', '?', '#', '@', '!'];
    s.chars().map(|c| {
        if c != ' ' && rng.gen_bool(0.25) {
            *noise.choose(rng).unwrap()
        } else {
            c
        }
    }).collect()
}

pub fn glitch_line() {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.08) {
        let noise: String = (0..40).map(|_| {
            let noise = vec!['▓', '░', '▒', '╳', '■'];
            *noise.choose(&mut rng).unwrap()
        }).collect();
        println!("{}", noise);
        thread::sleep(Duration::from_millis(40));
        print!("\x1B[1A\x1B[2K");
    }
}
