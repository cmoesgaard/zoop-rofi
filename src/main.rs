use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use copypasta_ext::copypasta::ClipboardProvider;
use copypasta_ext::x11_bin::ClipboardContext;
use rofi;
use rofi::pango::{FontSize, Pango};
use serde::{Deserialize, Serialize};
use serde_lexpr::{from_str, to_string};
use xdg;

const APPLICATION_NAME: &str = "zoop-rofi";
const ENTRY_FILE_NAME: &str = "entries.lisp";
const PROMPT: &str = "🤙";

#[derive(Deserialize, Serialize)]
struct Entry {
    name: String,
    aliases: Vec<String>,
}

fn prettify(entry: &Entry) -> String {
    let aliases = format!("({})", entry.aliases.join(", "));
    return format!(
        "{} {}", entry.name,
        Pango::new(aliases.as_str()).size(FontSize::Small).build()
    )
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APPLICATION_NAME).unwrap();
    let entry_file_path = xdg_dirs.place_config_file(ENTRY_FILE_NAME).unwrap();

    if !Path::exists(&entry_file_path.as_path()) {
        let mut file = File::create(&entry_file_path).expect("unable to create entry file");
        let default = vec![
            Entry {
                name: String::from("👉😎👉"),
                aliases: vec![String::from("zoop"), String::from("fingerguns")],
            }
        ];
        let default_str = to_string(&default).unwrap();
        file.write_all(default_str.as_bytes()).unwrap();
    }

    let input = fs::read_to_string(&entry_file_path).expect("failure opening entry file");
    let entries: Vec<Entry> = from_str(input.as_str()).expect("unable to parse entries");

    let sequences: Vec<String> = entries.iter()
        .map(|x| x.name.clone())
        .collect();

    let lines: Vec<String> = entries.iter()
        .map(prettify)
        .collect();

    let mut ctx = ClipboardContext::new().expect("unable to create clipboard context");
    match rofi::Rofi::new(&lines).pango().prompt(PROMPT).run_index() {
        Ok(choice) => ctx.set_contents(sequences[choice].to_owned()).unwrap(),
        Err(rofi::Error::Interrupted) => println!("Interrupted"),
        Err(e) => println!("Error: {}", e)
    }
}
