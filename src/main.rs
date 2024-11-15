use bobo_html_parser::{
    cli::{Action, CommandLineArgs},
    core::{parse_html, print_help, print_parsed_tree, read_html_from_file},
    error::HtmlParserError,
};
use structopt::StructOpt;

fn main() -> Result<(), HtmlParserError> {
    let CommandLineArgs { action, file } = CommandLineArgs::from_args();

    match action {
        Action::Parse => {
            let file_path = file.expect("Failed to find html file");
            let html = read_html_from_file(file_path)?;
            let result = parse_html(&html)?;

            print_parsed_tree(result);
        }
        Action::Help => {
            print_help();
        }
    }

    Ok(())
}
