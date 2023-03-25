use clap::{Arg, Command};

pub struct Application {
  cli: Command
}

impl Application {
    pub fn new() -> Self {
        let cli = Command::new("vt")
            .version("0.1")
            .author("kazuki-komori")
            .arg(
                Arg::new("test")
            );
        
        Self { cli }
    }

    pub fn run(&mut self) {
        // matcherで引数を読み取り
        let matcher =  self.cli.clone().get_matches();
        
        match matcher.get_one::<String>("test") {
            Some(s) => println!("{:?}", s),
            None => println!("None")
        }
    }
}