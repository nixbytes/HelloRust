use ferris_says::say; // Basic import
use std::env;
use std::io::{self, Read}; // Need this to read input in
use std::io::{stdout, BufWriter}; // Write to output

fn main() {
    // Define the args and collect in a string vec
    let args: Vec<String> = env::args().collect();
    let stdout = stdout();
    // list the first args for the stdout
    let input = &args[1];
    let message = String::from(input);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
