use easy_reader::EasyReader;
use regex::Regex;
use std::io::Error;

struct MdReader {
    lines_read: usize,
    line_buf: Vec<String>,
}

impl MdReader {
    fn new() -> MdReader {
        MdReader {
            lines_read: 0,
            line_buf: Vec::<String>::new(),
        }
    }

    fn generate_ind_md(&self) {
        println!("Lines read: {:?}", self.line_buf);
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

pub fn run(
    cmd_name: String,
    lines_to_read: usize,
    file: std::path::PathBuf,
    group: bool,
) -> Result<(), Error> {
    let re_capture = Regex::new(r"(?P<name>[:]\s\d{0,10}[:]\d[;])(?P<cmd>.*)").unwrap();

    let re_save = Regex::new(&format!(
        r"^:\s\d{{0,10}}[:]\d[;]({})\s[s]{{1}}\s{{0,}}\d{{0,}}$",
        cmd_name
    ))
    .unwrap();

    let re_save_group = Regex::new(&format!(
        r"(?P<zsh>^[:]\s\d{{0,10}}[:]\d[;])(?P<cmd_name>{})(?P<save>\s[s]{{1}}\s{{0,}})(?P<lines>\d{{0,}})",
        cmd_name
    ))
    .unwrap();

    let mut md_reader = MdReader::new();

    let file = std::fs::File::open(file)?;
    let mut reader = EasyReader::new(file)?;

    reader.eof();
    while let Some(line) = reader.prev_line()? {
        if re_save.is_match(&line) {
            if let Some(nl) = re_save_group.captures(&line).unwrap().name("lines") {
                for _ in 0..nl.as_str().parse::<usize>().unwrap_or(1) {
                    let line: &String = &reader.prev_line()?.unwrap();
                    let line = re_capture.captures(&line).unwrap();
                    md_reader
                        .line_buf
                        .insert(0, line.name("cmd").unwrap().as_str().to_owned());
                }
            }
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

#[cfg(test)]
mod test {
    use super::*;

    use assert_fs::prelude::*;
    #[test]
    fn test_run_multiline() {
        let cmd_name = String::from("rmd");
        let temp_file = assert_fs::NamedTempFile::new("histfile").unwrap();
        temp_file
            .write_str(
                ": 1645910774:0;echo \"save this_1\"\n\
: 1645910780:0;rmd s\n\
: 1645910785:0;echo \"ignore_this\"\n\
: 1645910786:0;echo \"ignore_this\"\n\
: 1645910790:0;echo \"save_this_2\"\n\
: 1645910810:0;echo \"save_this_2\"\n\
: 1645910813:0;rmd s 2\n\
: 1645910815:0;rmd -g last 2",
            )
            .unwrap();
        assert_eq!((), run(cmd_name, 3, temp_file.to_path_buf(), true).unwrap());
        /*
        ```sh
        echo "save this_1"
        echo "save_this_2"
        echo "save_this_2"
        ```
        */
        temp_file.close().unwrap();
    }

    #[test]
    fn test_run_single_line() {
        let cmd_name = String::from("rmd");
        let temp_file = assert_fs::NamedTempFile::new("histfile").unwrap();
        temp_file
            .write_str(
                ": 1645910774:0;echo \"save this\"\n\
: 1645910780:0;rmd s\n\
: 1645910785:0;echo \"ignore\"\n\
: 1645910786:0;echo \"ignore\"\n\
: 1645910790:0;echo \"ignore\"\n\
: 1645910810:0;echo \"save this\"\n\
: 1645910813:0;rmd s\n\
: 1645910815:0;rmd -g last 2",
            )
            .unwrap();
        assert_eq!((), run(cmd_name, 3, temp_file.to_path_buf(), true).unwrap());
        /*
        ```sh
        echo "save this"
        echo "save this"
        */
        temp_file.close().unwrap();
    }
}
