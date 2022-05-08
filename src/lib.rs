use easy_reader::EasyReader;
use regex::Regex;
use skim::prelude::*;
use skim::SkimItem;
use std::error::Error;

struct MdReader {
    line_buf: Vec<String>,
}

impl MdReader {
    fn new() -> MdReader {
        MdReader {
            line_buf: Vec::<String>::new(),
        }
    }

    fn generate_ind_md(&self) {
        self.line_buf
            .iter()
            .for_each(|l| println!("```sh\n{}\n```", l));
    }

    fn generate_group_md(&self) {
        let mut s: String = "```sh\n".to_string();
        self.line_buf
            .iter()
            .for_each(|l| s.push_str(&format!("{}\n", l)));
        s.push_str("```");
        println!("{}", s);
    }
}

pub fn doc(history: Vec<Arc<dyn SkimItem>>, group: bool) -> Result<(), Box<dyn Error>> {
    let mut md_reader = MdReader::new();
    let zsh_capture_cmd = Regex::new(r"(?P<name>[:]\s\d{0,10}[:]\d[;])(?P<cmd>.*)").unwrap();

    history.into_iter().for_each(|x| {
        let cmd = &x.output();
        let line = zsh_capture_cmd.captures(cmd).unwrap();
        md_reader
            .line_buf
            .push(line.name("cmd").unwrap().as_str().to_string());
    });

    match group {
        false => md_reader.generate_ind_md(),
        true => md_reader.generate_group_md(),
    };
    Ok(())
}
