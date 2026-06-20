use std::thread;
use std::time::Duration;

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
        println!("{}", line);
        thread::sleep(Duration::from_millis(120));
    }
}
