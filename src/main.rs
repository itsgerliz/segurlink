use clap::Parser;
use env_logger::Env;
use log::debug;

mod server;
mod client;
mod common;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(help = "Target IP address", required = true)]
    target_ip: String,
    #[arg(help = "Target TCP port", required = true)]
    target_port: u16,
    #[arg(help = "Run as server?", short = 's', long = "server", required = false)]
    server: bool
}

fn main() {
    // Parse args
    let args = CliArgs::parse();
    // Set default logging level to INFO
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    debug!("Intiated logger!");

    if args.server {
        // Init the server side
        server::init(&(args.target_ip.as_str(), args.target_port))
    } else {
        // Init the client side
        todo!()
    }
}