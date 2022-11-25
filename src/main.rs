use anyhow::Result;
use log::error;

mod cli;
mod collector;
mod core;
use cli::get_cli;
use collector::get_collectors;

fn main() -> Result<()> {
    let mut cli = get_cli()?.build()?;

    let command = cli.get_subcommand_mut()?;
    match command.name() {
        "collect" => {
            let mut collectors = get_collectors()?;
            collectors.register_cli(command.dynamic_mut().unwrap())?;
            let config = cli.run()?;
            collectors.init(&config)?;
            collectors.start(&config)?;
        }
        _ => {
            error!("not implemented");
        }
    }
    Ok(())
}
