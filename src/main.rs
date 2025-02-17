use parser::Parser;
use sources::{Source, SourceMap};

mod error;
mod parser;
mod sources;

fn main() {
    let mut sources = SourceMap::default();

    let mut parser = Parser::new(Source::new("glib.shid"), &mut sources);

    let ast = parser.parse_expr();
    println!("{:#?}", ast);

    for i in parser.errors {
        i.into_report().display(&sources);
    }
}
