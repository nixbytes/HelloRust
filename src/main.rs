use ferris_says::say; // basic import
use std::io::{self, Read}; // Need this to read input in
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input) // get the input by reference   
        .expect("failed to get input"); //expect is used for checking for errors
    let message = String::from(input);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
