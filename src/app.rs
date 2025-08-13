use std::io;

use crate::mode::ModeKind;

pub fn app<Lambda: Fn(String)>(cycle: Lambda) {
    match get_mode() {
        ModeKind::File(path) => match std::fs::read_to_string(path) {
            Ok(content) => cycle(content),
            Err(e) => {
                eprintln!(
                    "(going in terminal, instead
                    of file mode due to: Error({e})"
                );

                terminal(&cycle);
            }
        },
        ModeKind::Terminal => terminal(&cycle),
    };
}

fn terminal<Lambda: Fn(String)>(cycle: &Lambda) {
    loop {
        cycle(read_line());
    }
}

fn read_line() -> String {
    let mut input: String = String::from("");
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn get_mode() -> ModeKind {
    let mut args = std::env::args();

    if args.next().is_none() {
        panic!("No arguments given!")
    }

    match args.next() {
        Some(path) => ModeKind::File(path),
        None => ModeKind::Terminal,
    }
}
