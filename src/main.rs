use clap::Parser;
use futures_util::stream::StreamExt;

/// Command voltage and current from a DeltaQ ICL 1500-058 charger.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(long, short = 'v')]
    volts: f32,

    #[arg(long, short = 'a')]
    amps: f32,

    #[arg(long, short = 't', default_value_t = 20.0)]
    temperature: f32,

    #[arg(long, short = 's', default_value_t = 50)]
    soc: u8,

    #[arg(long, short = 'c', default_value_t = String::from("can0"))]
    can_interface: String,
}

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    let args = Args::parse();
    println!("config: {args:#?}");

    let mut can_socket_rx = tokio_socketcan::CANSocket::open(&args.can_interface)?;
    let can_socket_tx = tokio_socketcan::CANSocket::open(&args.can_interface)?;

    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(1));
    tokio::pin!(timeout);

    let (ctrlc_tx, mut ctrlc_rx) = tokio::sync::mpsc::channel::<()>(1);
    ctrlc::set_handler(move || {
        let _ = ctrlc_tx.try_send(());
    })?;

    loop {
        tokio::select! {
            _ = ctrlc_rx.recv() => {
                println!();
                println!("Goodbye!");
                // Shut down the charger.
                let _ = charger::send_command(&can_socket_tx, 0.0, 0.0, args.temperature, args.soc).await;
                break;
            }

            maybe_frame = can_socket_rx.next() => {
                match maybe_frame {
                    Some(Ok(_frame)) => {
                        // let _ = handle_can_frame(frame);
                    }
                    _ => ()
                }
            }
            _ = &mut timeout => {
                let _ = charger::send_command(&can_socket_tx, args.volts, args.amps, args.temperature, args.soc).await;
                timeout.set(tokio::time::sleep(tokio::time::Duration::from_secs(1)));
            }
        }
    }

    Ok(())
}
