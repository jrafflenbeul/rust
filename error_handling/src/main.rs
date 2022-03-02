use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? Operator
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? Operator
    Ok(s)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_maximum_shortness() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Function returns Some(char) or None -> ? Operator usable here
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// main function which returns Result
// main can only return types which implementt the std::process::Termination trait
// (only available in nightly Rust right now!)
fn main() -> Result<(), Box<dyn Error>> {
    // panic
    panic!("crash and burn");

    let v = vec![1, 2, 3];

    v[99];

    // error handling with Result<T, E>
    let f1 = File::open("hello.txt");

    let f1 = match f1 {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error)
    };


    let f2 = File::open("hello.txt");
    let f2 = match f2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    
    let f3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap returns Ok on success and panics on error
    let f4 = File::open("hello.txt").unwrap();

    // add custom error message with except()
    let f5 = File::open("hello.txt").expect("Failed to open hello.txt");

    // ? Operator
    
    let f = File::open("hello.txt")?; // -> not possible, if main returns (), not Result, Option or another type that implements FromResidual

    Ok(())
}

// Guessing game with contract (value between 1 and 100)
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}