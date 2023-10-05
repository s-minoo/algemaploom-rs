use clap::{arg, Command};

pub const TRANSLATOR_VERSION: &'static str = "0.1";

pub struct Cli {
    pub cmd: Command,
}

impl Cli {
    pub fn new() -> Cli {
        let cmd = Command::new("RML-meamer translator ")
            .version(TRANSLATOR_VERSION)
            .author("Sitt Min Oo")
            .about("Translates RML documents to execution plans composed by algebraic mapping operators")
            .subcommand_required(true)
            .propagate_version(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("file")
                         .about("translate a single RML document")
                         .arg(arg!(<RML_DOCUMENT> "the RML document to be translated"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("folder")
                         .about("translate all RML documents under the given folder")
                         .arg(arg!(<FOLDER> "the folder containing several RML documents"))
                         .arg_required_else_help(true))
            .arg(arg!(-o --output-folder-suffix <OUTPUT_FOLDER_SUFFIX> "The output folder suffix"));

        Self { cmd }
    }
}
