use std::error::Error;
use std::fs;
extern crate skim;
use skim::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

fn main() -> Result<(), Box<dyn Error>> {
    let options = SkimOptionsBuilder::default()
        .preview_window(Some("up:10%"))
        .preview(Some(""))
        .header(Some("SELECT:    enter     | EXIT:         esc\n\
                      PAGE UP:   page up   | PAGE DOWN:    page down \n\
                      SELECT UP: shift+tab | SELECT DOWN:  tab\n\
                      ──────────────────────────────────────────────────────────────────────────────",
        ))
        .multi(true)
        .build()
        .unwrap();

    let hist_file =
        std::env::var_os("HISTFILE").unwrap_or_else(|| std::ffi::OsString::from(".zsh_history"));
    #[allow(deprecated)]
    let home_dir = std::env::home_dir().expect("could not determine a home directory");
    let path = home_dir.join(&hist_file);
    let file_len = fs::metadata(&path)?.len();
    let mut f = File::open(path)?;

    if file_len > 4096 {
        f.seek(SeekFrom::End(-4096))?;
    }

    let mut bf_path = BufReader::new(f);

    let mut temp_buffer = String::new();
    bf_path.read_line(&mut temp_buffer).expect("EOF");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(bf_path);

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    rmd::doc(selected_items, true)?;

    Ok(())
}
