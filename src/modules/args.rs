use std::env::args;

pub struct Args {
    pub input: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Args {
        let args = args().collect::<Vec<String>>();
        Args {
            input: match args.get(1) {
                Some(s) => s.to_owned(),
                None => panic!("Enter a input!"),
            },
            output: match args.get(2) {
                Some(s) => s.to_owned(),
                None => panic!("Enter a output!"),
            },
        }
    }
}
