use clap::{Parser, Subcommand};

/// Command voltage and current from a DeltaQ XV3300 charger.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, short = 'c', default_value_t = String::from("can0"))]
    can_interface: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    CanListenOnly {
        /// Get or set XV3300 "CAN Listen Only" setting.
        set: Option<bool>,
    },

    #[command()]
    AcCurrentLimit {
        /// Get or set XV3300 "AC Current Limit" setting.
        set: Option<f32>,
    },
}

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    let args = Args::parse();
    println!("config: {args:#?}");

    let (tx, rx) = zencan_client::open_socketcan(&args.can_interface).unwrap();
    let mut sdo_client = zencan_client::SdoClient::new_std(0x64, tx, rx);

    match args.command {
        Commands::CanListenOnly { set: Some(val) } => {
            deltaq_xv3300::set_can_listen_only(&mut sdo_client, val).await?;
        }
        Commands::CanListenOnly { set: None } => {
            let can_listen_only = deltaq_xv3300::get_can_listen_only(&mut sdo_client).await?;
            println!("CAN Listen Only: {can_listen_only}");
        }
        Commands::AcCurrentLimit { set: Some(val) } => {
            deltaq_xv3300::set_ac_current_limit(&mut sdo_client, val).await?;
        }
        Commands::AcCurrentLimit { set: None } => {
            let ac_current_limit = deltaq_xv3300::get_ac_current_limit(&mut sdo_client).await?;
            println!("AC Current Limit: {ac_current_limit:.3} A");
        }
    }

    Ok(())
}
