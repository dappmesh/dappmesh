use clap::{Args, Subcommand};

#[derive(Subcommand)]
#[command(subcommand_help_heading = "Helm Commands")]
pub(crate) enum HelmCommands {
    /// Import helm chart from source to destination
    Import(ImportArgs),
}

#[derive(Args)]
pub(crate) struct ImportArgs {
    /// Source URL or path of the helm chart
    #[arg(short, long, value_name = "SOURCE")]
    pub(crate) source: String,

    /// Destination path for the imported helm chart
    #[arg(short, long, value_name = "DESTINATION")]
    pub(crate) destination: String,
}

pub(crate) struct Helm<'a> {
    source: &'a String,
    destination: &'a String,
}

impl<'a> Helm<'a> {
    pub(crate) fn new(source: &'a String, destination: &'a String) -> Self {
        Self {
            source,
            destination,
        }
    }

    pub(crate) fn import(&self) {
        println!("Importing from {} to {}", self.source, self.destination);
    }
}