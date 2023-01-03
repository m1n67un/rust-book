/// I/O프로젝트: 커맨드 라인 프로그램 만들기
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

/*
    cargo run the poem.txt
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
        Running `target\debug\greprs.exe the poem.txt`
    ["target\\debug\\greprs.exe", "the", "poem.txt"]
    Searching for the
    In file poem.txt
    With text:
    I'm nobody! Who are you?
    Are you nobody, too?
    Then there's a pair of us — don't tell!
    They'd banish us, you know.

    How dreary to be somebody!
    How public, like a frog
    To tell your name the livelong day
    To an admiring bog!
*/

///: 첫번째 리팩토링
fn main1() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config1(&args);

    // ...snip...
}

fn parse_config1(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

///: 두번째 리팩토링
fn main2() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config2(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    // ...snip...
}

struct Config {
    query: String,
    filename: String,
}


impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enoutgh arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }

    fn new2(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn parse_config2(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

///: 세번째 리팩토링
fn main3() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // ...snip...
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
// ...snip...

use std::error::Error;

// ...snip...

fn run2(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}