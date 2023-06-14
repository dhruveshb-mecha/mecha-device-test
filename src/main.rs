use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod base_cli;

use base_cli::DeviceConfig;
use clap::{Args, Parser, Subcommand, ValueEnum};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "test")]
#[command(about = "A device hardware testing cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

const DEFAULT_COVERAGE: &str = "all";

#[derive(Debug, Subcommand)]
enum Commands {
    /// Clones repos
    #[command(arg_required_else_help = true)]
    Test {
        ///  tests to run, if not specified defaults to all
        #[arg(short, long, default_value_t=DEFAULT_COVERAGE.to_string())]
        coverage: String,

        /// device that will be used to run the tests
        #[arg(short, long, required = true)]
        device: String,

        /// profile path that conains device specific configuration
        #[arg(short, long, required = true)]
        profile: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Test {
            coverage,
            device,
            profile,
        } => {
            let profile_file = File::open(&profile).expect("Failed to open config file");
            let reader = BufReader::new(profile_file);

            let device_config: DeviceConfig = serde_yaml::from_reader(reader).unwrap();

            //println!("device_config: {:?}", device_config);
            println!(
                "device_config: {:?}",
                device_config.interfaces.display.device
            );
        }
    }
}
