use clap::{Parser, Subcommand};
use mdbook::MDBook;
use std::process::Command as ProcessCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Build the explanation as an HTML book")]
    Build,
    #[command(about = "Serve the explanation as an HTML book")]
    Serve,
}

fn main() {
    colog::init();

    let arguments = Args::parse();

    let book_root = env!("CARGO_MANIFEST_DIR");
    let book = MDBook::load(book_root).expect("Unable to load book");

    match &arguments.command {
        Command::Build => {
            book.build().expect("Unable to build book");
        }
        Command::Serve => {
            ProcessCommand::new("mdbook")
                .arg("serve")
                .arg(book_root)
                .spawn()
                .expect("Unable to serve book")
                .wait()
                .expect("Unable to serve book");
        }
    };
}
