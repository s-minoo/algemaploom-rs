use std::fs::File;
use std::io::{stdin, stdout, BufReader, IsTerminal, Read};
use std::path::PathBuf;
use std::process::exit;

use clap::{CommandFactory, Parser};
use shexml_interpreter::errors::{ShExMLError, ShExMLResult};
use shexml_interpreter::parse_string;

#[derive(Debug, Parser)]
#[command(
    about = "Parses ShExML format into JSON object. You can also pipe a ShExML file for parsing"
)]
struct Args {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

pub fn main() -> ShExMLResult<()> {
    let args = Args::parse();
    let mut cmd = Args::command();

    let mut shexml_doc_string = String::new();

    if let Some(file) = args.file {
        File::open(file)?.read_to_string(&mut shexml_doc_string)?;
    } else {
        if stdin().is_terminal() {
            cmd.print_help()?;
            exit(2);
        }

        let mut stdin_buf_reader = BufReader::new(std::io::stdin());
        stdin_buf_reader.read_to_string(&mut shexml_doc_string)?;
    }

    let shexml_doc = parse_string(shexml_doc_string)?;
    let shexml_json =
        serde_json::to_string_pretty(&shexml_doc).map_err(|err| {
            ShExMLError {
                dbg_msg: format!("{:?}", err),
                msg:     format!("{}", err),
                err:
                    shexml_interpreter::errors::ShExMLErrorType::SerdeError,
            }
        })?;

    println!("{}", shexml_json);
    Ok(())
}
