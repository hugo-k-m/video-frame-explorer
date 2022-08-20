mod clip;
mod parser;

use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = parser::subcommands::Parser::from_args();
    let parser_status = opts.apply_parser_args()?;

    println!("{}", parser_status);

    Ok(())
}
