use anyhow::Result;
use clap::{builder::EnumValueParser, Arg, Command, ValueEnum};
use solana_hexagonal_poh::prelude::*;
use spinners::{Spinner, Spinners};

#[derive(Clone, Debug, ValueEnum)]
pub enum Mode {
    Node,
}

#[tokio::main]
async fn main() -> Result<()> {
    // set cli commands
    let cmd = Command::new("shpoh")
        .version("0.1.0")
        .about("Solana Hexagonal Proof of History")
        .arg_required_else_help(true)
        .arg(
            Arg::new("BASEURL")
                .long("base-url")
                .short('b')
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::new("PORT")
                .long("port")
                .short('p')
                .default_value("8080"),
        )
        .arg(
            Arg::new("MODE")
                .required(true)
                .long("mode")
                .short('m')
                .value_parser(EnumValueParser::<Mode>::new()),
        );

    // get flag values
    let matches = cmd.get_matches();

    // unwrap ok here, required/default_value ensures filled
    let mode = matches.get_one::<Mode>("MODE").unwrap();
    let base_url = matches.get_one::<String>("BASEURL").unwrap();
    let port = matches.get_one::<String>("PORT").unwrap();

    match mode {
        Mode::Node => {
            let mut node = Node::new(base_url, port);
            node.run().await
        }
    }
}
