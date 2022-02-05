use std::ffi::OsString;

use clap::Parser;
use espmonitor::{run, AppArgs, Framework};
use klask::Settings;

use crate::types::chip::Chip;
use crate::types::serial::Serial;

mod types;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the serial device
    #[clap(long, arg_enum, required = true, ignore_case = true)]
    pub serial: Serial,

    /// Which ESP chip to target
    #[clap(long, arg_enum, required = true, ignore_case = true)]
    pub chip: Chip,

    /// Baud rate of serial device
    #[clap(long, default_value = "115200")]
    pub speed: usize,

    /// Disable chip reset on run
    #[clap(long)]
    pub disable_reset: bool,

    /// Path to executable matching what is on the device
    #[clap(long)]
    pub bin: Option<OsString>,
}

impl Args {
    fn get_app_args(&self) -> AppArgs {
        AppArgs {
            bin: self.bin.to_owned(),
            chip: self.chip.to_esp_chip(),
            framework: Framework::default(),
            reset: !self.disable_reset,
            serial: self.serial.to_string(),
            speed: Some(self.speed),
        }
    }
}

fn main() {
    klask::run_derived::<Args, _>(Settings::default(), |args| {
        // dbg!(&args);
        //
        // let em_args = args.get_app_args();
        // dbg!(&em_args);

        match run(args.get_app_args()) {
            Ok(_) => (),
            Err(err) => eprintln!("[Error] {err}"),
        }
    });
}
