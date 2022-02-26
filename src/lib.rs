use easy_reader::EasyReader;
use regex::Regex;
use std::io::Error;

struct MdReader {
    lines_read: usize,
    line_buf: Vec<String>,
}

impl MdReader {
    fn new() -> MdReader {
        return MdReader {
            lines_read: 0,
            line_buf: Vec::<String>::new(),
        };
    }

    fn generate_ind_md(&self) -> () {
        println!("Lines read: {:?}", self.line_buf);
        self.line_buf
            .iter()
            .for_each(|l| println!("```sh\n{}\n```", l));
    }

    fn generate_group_md(&self) -> () {
        let mut s: String = "```sh\n".to_string();
        self.line_buf
            .iter()
            .for_each(|l| s.push_str(&format!("{}\n", l).to_string()));
        s.push_str("```");
        println!("{}", s);
    }
}

pub fn run(lines_to_read: usize, file: std::path::PathBuf, group: bool) -> Result<(), Error> {
    let re_capture = Regex::new(r"(?P<name>[:]\s\d{0,10}[:]\d[;])(?P<cmd>.*)").unwrap();
    let cmd_name = std::env::args().next();

    let re_save = Regex::new(&format!(
        r"^:\s\d{{0,10}}[:]\d[;]({})\s[s]{{1}}$",
        cmd_name.unwrap()
    ))
    .unwrap();

    let mut md_reader = MdReader::new();

    let file = std::fs::File::open(file)?;
    let mut reader = EasyReader::new(file)?;

    reader.eof();
    while let Some(line) = reader.prev_line()? {
        if re_save.is_match(&line) {
            let line: &String = &reader.prev_line()?.unwrap();
            let line = re_capture.captures(line).unwrap();
            md_reader
                .line_buf
                .insert(0, line.name("cmd").unwrap().as_str().to_owned());
            md_reader.lines_read += 1;
        }

        if md_reader.lines_read == lines_to_read {
            break;
        }
    }

    if group {
        md_reader.generate_group_md();
    } else {
        md_reader.generate_ind_md();
    }

    Ok(())
}
