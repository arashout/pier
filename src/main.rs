use std::process;
use structopt::StructOpt;

use pier::{
    cli::{Cli, CliSubcommand},
    open_editor,
    script::Script,
    Pier, Result,
};

fn main() {
    let opt = Cli::from_args();

    match handle_subcommands(opt) {
        Ok(status) => {
            if let Some(status) = status {
                let code = status.code().unwrap_or(0);
                process::exit(code)
            } else {
                process::exit(0)
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
}

/// Handles the commandline subcommands
fn handle_subcommands(cli: Cli) -> Result<Option<process::ExitStatus>> {
    let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
    //let interpreter = config.get_interpreter();
    if let Some(subcmd) = cli.cmd {
        match subcmd {
            CliSubcommand::Add {
                command,
                alias,
                description,
                tags,
            } => {
                pier.add_script(Script {
                    alias,
                    description,
                    command: match command {
                        Some(cmd) => cmd,
                        None => open_editor(None)?,
                    },
                    tags,
                    reference: None,
                })?;
                pier.write()?;
            }

            CliSubcommand::Edit { alias } => {
                pier.edit_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::Remove { alias } => {
                pier.remove_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::Show { alias } => {
                let script = pier.fetch_script(&alias)?;
                println!("{}", script.command);
            }
            CliSubcommand::List {
                list_aliases,
                tags,
                cmd_full,
                cmd_width,
            } => {
                if list_aliases {
                    pier.list_aliases(tags)?
                } else {
                    pier.list_scripts(tags, cmd_full, cmd_width)?
                }
            }
            CliSubcommand::Run { alias, args } => {
                let exit_code = pier.run_script(&alias, args)?;
                return Ok(Some(exit_code));
            }
        };
    } else {
        let alias = &cli.alias.expect("Alias is required unless subcommand.");
        let exit_code = pier.run_script(alias, cli.args)?;
        return Ok(Some(exit_code));
    }

    Ok(None)
}
