use cliparser::parse;
use cliparser::types::CliSpec;

fn main() {
    let cli_spec = CliSpec::new();
    let cli_parsed = parse(&vec![], &cli_spec).expect("Unable to parse command line");
    println!("{:#?}", &cli_parsed); // todo
}
