use argparse::{ArgumentParser, StoreOption};

pub mod advent;
pub mod util;

fn main() {
    let mut day: Option<u32> = None;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Advent of Code 2023");
        parser.refer(&mut day).add_option(
            &["-d", "--day"],
            StoreOption,
            "number of challenge to run",
        );
        parser.parse_args_or_exit();
    }
    match day {
        Some(ref day) => match advent::solve(*day) {
            Ok(_) => {}
            Err(e) => println!("error: {}", e),
        },
        None => println!("--day is required"),
    }
}
