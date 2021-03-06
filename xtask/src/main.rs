use pico_args::Arguments;
use xtask::{
    codegen::{self, Mode},
    docgen,
    glue::pushd,
    project_root, run_rustfmt, Result,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "codegen" => {
            args.finish()?;
            codegen::generate_parser_tests(Mode::Overwrite)?;
            Ok(())
        }
        "syntax" => {
            args.finish()?;
            codegen::generate_syntax(Mode::Overwrite)?;
            Ok(())
        }
        "format" => {
            args.finish()?;
            run_rustfmt(Mode::Overwrite)
        }
        "docgen" => {
            args.finish()?;
            docgen::run();
            Ok(())
        }
        _ => {
            eprintln!(
                "\
cargo xtask
Run custom build command.
USAGE:
    cargo xtask <SUBCOMMAND>
SUBCOMMANDS:
    format
    codegen
    syntax
    docgen"
            );
            Ok(())
        }
    }
}
