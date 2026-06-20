mod filesystem;
mod shell;
mod glitch;

fn main() {
    glitch::boot();
    shell::run();
}
