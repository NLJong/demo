use clap::{Parser, Subcommand};
use codespan_reporting::term::termcolor::ColorChoice;
use mdbook::{errors::Error, renderer::RenderContext, MDBook, Renderer};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Build the explanation as an HTML book")]
    Build,
}

struct LinkCheckRenderer;

impl LinkCheckRenderer {
    fn new() -> Self {
        LinkCheckRenderer
    }
}

impl Renderer for LinkCheckRenderer {
    fn name(&self) -> &str {
        "linkcheck"
    }

    fn render(&self, ctx: &RenderContext) -> Result<(), Error> {
        let mut ctx = ctx.clone();
        ctx.config
            .set("output.linkcheck.follow-web-links", true)
            .expect("Unable to set linkcheck config");

        let cache_file = ctx.destination.join("linkcheck_cache.json");
        let cache_file = Some(cache_file.as_path());

        mdbook_linkcheck::run(cache_file, ColorChoice::Auto, &ctx, None)
    }
}

fn main() {
    colog::init();

    let arguments = Arguments::parse();
    let book_root = env!("CARGO_MANIFEST_DIR");

    let mut book = MDBook::load(book_root).expect("Unable to load book");
    let book = book
        .with_preprocessor(mdbook_alerts::Preprocessor)
        .with_renderer(LinkCheckRenderer::new());

    match &arguments.command {
        Command::Build => {
            book.build().expect("Unable to build book");
        }
    };
}
