use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod base_cli;

use base_cli::DeviceConfig;
use clap::{Parser, Subcommand};

use test_base::{TestAssertion, TestRunner};
use tests::camera::capture_image::CameraImageCapture;
use tests::display::display_brightness::DisplayBrightness;
use tests::display::display_detect::DisplayDetect;

mod test_base;

mod tests {
    pub mod display {
        pub mod display_brightness;
        pub mod display_detect;
    }
    pub mod camera {
        pub mod capture_image;
    }
}

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

///filter test cases by coverage
fn filter_test_cases(
    test_cases: Vec<(&str, Box<dyn TestAssertion>)>,
    filter: &str,
) -> Vec<Box<dyn TestAssertion>> {
    test_cases
        .into_iter()
        .filter(|(param, _)| param == &filter)
        .map(|(_, assertion)| assertion)
        .collect()
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

            let device_config: DeviceConfig =
                serde_yaml::from_reader(reader).expect("unable to rad yaml file");

            let test_cases: Vec<(&str, Box<dyn TestAssertion>)> = vec![
                (
                    "display",
                    Box::new(DisplayDetect {
                        device: device_config.interfaces.display.device.clone(),
                    }),
                ),
                (
                    "display",
                    Box::new(DisplayBrightness {
                        device: device_config.interfaces.display.device.clone(),
                    }),
                ),
                ("camera", Box::new(CameraImageCapture)),
                // we can add all test case over here.
                // (
                //     "led",
                //     Box::new(LedDetect {
                //         device: device_config.interfaces.display.device.clone(),
                //     }),
                // ),
            ];

            let mut suit: Vec<Box<dyn TestAssertion>> = Vec::new();

            match coverage.as_str() {
                "all" => {
                    suit = test_cases
                        .into_iter()
                        .map(|(_, assertion)| assertion)
                        .collect();
                }
                "display" => {
                    suit = filter_test_cases(test_cases, "display");
                }
                "led" => {
                    suit = filter_test_cases(test_cases, "led");
                }
                _ => panic!("Invalid coverage: {}", coverage),
            }

            let mut test_suite = TestRunner {
                suit,
                test_count: 0,
                test_passed: 0,
                test_failed: 0,
                test_runner: || println!("Running tests"),
            };

            test_suite.run().unwrap();
        }
    }
}
