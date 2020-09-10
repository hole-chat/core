use std::io;

pub fn get_stdin () -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    return input

}

