use clap::Parser;
use cli::SubCommands;
use rive_models::authentication::Authentication;

mod cli;
mod platforms;

#[cfg(feature = "lastfm")]
use crate::platforms::{lastfm::LastFM, Platform, Status};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Args::parse();

    let status = Status {
        template: cli.status_template,
        idle: cli.status_idle,
    };

    match cli.command {
        SubCommands::LastFM {
            user,
            api_key,
            check_delay,
        } => {
            let rive_client = rive_http::Client::new(Authentication::SessionToken(cli.token));

            LastFM {
                user,
                api_key,
                check_delay,
                ..Default::default()
            }
            .initialise()
            .await?
            .event_loop(rive_client, status)
            .await;
        }
    }

    Ok(())
}
