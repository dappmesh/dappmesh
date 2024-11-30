mod command;

use crate::command::helm::{Helm, HelmCommands};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	#[command(subcommand)]
	Helm(HelmCommands),
}

fn main() {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Helm(helm_cmd) => match helm_cmd {
			HelmCommands::Import(args) => {
				let helm = Helm::new(&args.source, &args.destination);
				helm.import();
			}
		},
	}
}
