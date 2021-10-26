//! This module implements the `chunk` CLI command
use generated_types::google::FieldViolation;
use influxdb_iox_client::{
    connection::Connection,
    management::{self, ListChunksError},
};
use structopt::StructOpt;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Error listing chunks: {0}")]
    ListChunkError(#[from] ListChunksError),

    #[error("Error interpreting server response: {0}")]
    ConvertingResponse(#[from] FieldViolation),

    #[error("Error rendering response as JSON: {0}")]
    WritingJson(#[from] serde_json::Error),

    #[error("Error connecting to IOx: {0}")]
    ConnectionError(#[from] influxdb_iox_client::connection::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Manage IOx chunks
#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(subcommand)]
    command: Command,
}

/// List the chunks for the specified database in JSON format
#[derive(Debug, StructOpt)]
struct List {
    /// The name of the database
    db_name: String,
}

/// All possible subcommands for chunk
#[derive(Debug, StructOpt)]
enum Command {
    List(List),
}

pub async fn command(connection: Connection, config: Config) -> Result<()> {
    match config.command {
        Command::List(get) => {
            let List { db_name } = get;

            let mut client = management::Client::new(connection);

            let chunks = client.list_chunks(db_name).await?;

            serde_json::to_writer_pretty(std::io::stdout(), &chunks)?;
        }
    }

    Ok(())
}