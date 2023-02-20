use ferris_says::say; // Basic import
use std::env::args;
use std::io::{self, Read}; // Need this to read input in
use std::io::{stdout, BufWriter}; // Write to output

fn main() {
    // Define the vars for the in/output

    let stdout = stdout();
    let mut input = std::env::args().nth(1).expect("no pattern give text");
    //let args = Cli { pattern: pattern };

    io::stdin()
        .read_to_string(&mut input) // get the input by reference
        .expect("failed to get input"); //expect is used for checking for errors
    let message = String::from(input);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
