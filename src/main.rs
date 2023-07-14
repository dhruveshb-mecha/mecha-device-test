use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod base_cli;

use base_cli::DeviceConfig;
use clap::{Parser, Subcommand};

use mecha_mic::AudioInterface;
use test_base::{TestAssertion, TestRunner};
use tests::adc::adc_data::AdcTest;
use tests::battery::battery_charging::BatteryCharging;
use tests::battery::battery_detect::BatteryDetect;
use tests::camera::capture_image::CameraImageCapture;
use tests::cpu::cpu_stress_test::CpuStressTest;
// use tests::camera::capture_image::CameraImageCapture;
use tests::display::display_brightness::DisplayBrightness;
use tests::display::display_detect::DisplayDetect;
use tests::gyro::gyro_data::GyroData;
use tests::gyro::gyro_detect::GyroDetect;
use tests::led::{led_color_test, led_detect};
use tests::mic::audio_playback::AudioPlayBack;
use tests::mic::audio_record::AudioRecord;
use tests::powertests::power_test1::{self, PowerTest1};
use tests::powertests::power_test2::PowerTest2;
use tests::powertests::power_test3::PowerTest3;
use tests::powertests::power_test4::PowerTest4;
use tests::powertests::power_test5::PowerTest5;
use tests::pwm::pwm_blink_led::PWM;
use tests::trust_ic::detect_trust_ic::TrustIcDetectionTest;

mod test_base;

mod tests {
    pub mod display {
        pub mod display_brightness;
        pub mod display_detect;
    }
    pub mod camera {
        pub mod capture_image;
    }

    pub mod battery {
        pub mod battery_charging;
        pub mod battery_detect;
    }

    pub mod gyro {
        pub mod gyro_data;
        pub mod gyro_detect;
    }

    pub mod mic {
        pub mod audio_playback;
        pub mod audio_record;
    }

    pub mod led {
        pub mod led_color_test;
        pub mod led_detect;
    }

    pub mod pwm {
        pub mod pwm_blink_led;
    }

    pub mod adc {
        pub mod adc_data;
    }

    pub mod cpu {
        pub mod cpu_stress_test;
        // pub mod cpu_temp;
    }

    pub mod trust_ic {
        pub mod detect_trust_ic;
    }
    pub mod powertests {
        pub mod power_test1;
        pub mod power_test2;
        pub mod power_test3;
        pub mod power_test4;
        pub mod power_test5;
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
    /// test for mecha device
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
                // add battery test cases over here
                (
                    "battery",
                    Box::new(BatteryDetect {
                        device: device_config.interfaces.display.device.clone(),
                    }),
                ),
                (
                    "battery",
                    Box::new(BatteryCharging {
                        device: device_config.interfaces.display.device.clone(),
                    }),
                ),
                // add gyro test cases over here
                (
                    "gyro",
                    Box::new(GyroDetect {
                        x_axis_path: device_config.interfaces.gyroscope.x_axis.clone(),
                        y_axis_path: device_config.interfaces.gyroscope.y_axis.clone(),
                        z_axis_path: device_config.interfaces.gyroscope.z_axis.clone(),
                    }),
                ),
                (
                    "gyro",
                    Box::new(GyroData {
                        x_axis_path: device_config.interfaces.gyroscope.x_axis.clone(),
                        y_axis_path: device_config.interfaces.gyroscope.y_axis.clone(),
                        z_axis_path: device_config.interfaces.gyroscope.z_axis.clone(),
                    }),
                ),
                ("audio", Box::new(AudioRecord)),
                ("audio", Box::new(AudioPlayBack)),
                (
                    "led",
                    Box::new(led_detect::LedDetect {
                        red: device_config.interfaces.led.red_led.clone(),
                        green: device_config.interfaces.led.green_led.clone(),
                        blue: device_config.interfaces.led.blue_led.clone(),
                    }),
                ),
                (
                    "led",
                    Box::new(led_color_test::LedColorTest {
                        red: device_config.interfaces.led.red_led.clone(),
                        green: device_config.interfaces.led.green_led.clone(),
                        blue: device_config.interfaces.led.blue_led.clone(),
                    }),
                ),
                //pwm test case
                ("pwm", Box::new(PWM)),
                //adc test case
                (
                    "adc",
                    Box::new(AdcTest {
                        channel_1_path: device_config.interfaces.adc.channel_1,
                        channel_2_path: device_config.interfaces.adc.channel_2,
                        sampling_frequency: device_config.interfaces.adc.sampling_frequency,
                    }),
                ),
                ("cpu", Box::new(CpuStressTest)),
                ("trust_ic", Box::new(TrustIcDetectionTest)),
                (
                    "power_test_1",
                    Box::new(PowerTest1 {
                        display_path: device_config.interfaces.display.device.clone(),
                        camera_path: device_config.interfaces.camera.device.clone(),
                        current_now: device_config.interfaces.battery.current.clone(),
                    }),
                ),
                (
                    "power_test_2",
                    Box::new(PowerTest2 {
                        display_path: device_config.interfaces.display.device.clone(),
                        camera_path: device_config.interfaces.camera.device.clone(),
                        current_now: device_config.interfaces.battery.current.clone(),
                    }),
                ),
                (
                    "power_test_3",
                    Box::new(PowerTest3 {
                        display_path: device_config.interfaces.display.device.clone(),
                        camera_path: device_config.interfaces.camera.device.clone(),
                        current_now: device_config.interfaces.battery.current.clone(),
                    }),
                ),
                (
                    "power_test_4",
                    Box::new(PowerTest4 {
                        display_path: device_config.interfaces.display.device.clone(),
                        camera_path: device_config.interfaces.camera.device.clone(),
                        current_now: device_config.interfaces.battery.current.clone(),
                        audio_path: device_config.interfaces.audio.audio_file.clone(),
                    }),
                ),
                // (
                //     "power_test_5",
                //     Box::new(PowerTest5 {
                //         display_path: device_config.interfaces.display.device.clone(),
                //         camera_path: device_config.interfaces.camera.device.clone(),
                //     }),
                // ),
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
                        .filter(|(param, _)| !param.starts_with("powertest"))
                        .map(|(_, assertion)| assertion)
                        .collect();
                }
                "display" => {
                    suit = filter_test_cases(test_cases, "display");
                }
                "led" => {
                    suit = filter_test_cases(test_cases, "led");
                }
                "battery" => {
                    suit = filter_test_cases(test_cases, "battery");
                }
                "gyro" => {
                    suit = filter_test_cases(test_cases, "gyro");
                }
                "audio" => {
                    suit = filter_test_cases(test_cases, "audio");
                }
                "pwm" => {
                    suit = filter_test_cases(test_cases, "pwm");
                }
                "adc" => {
                    suit = filter_test_cases(test_cases, "adc");
                }
                "cpu" => {
                    suit = filter_test_cases(test_cases, "cpu");
                }
                "trust_ic" => {
                    suit = filter_test_cases(test_cases, "trust_ic");
                }
                _ if coverage.starts_with("powertest") => {
                    let powertest_name = coverage.strip_prefix("powertest").unwrap();
                    let power_tetcase_name = format!("power_test_{}", powertest_name);
                    suit = filter_test_cases(test_cases, &power_tetcase_name);
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
