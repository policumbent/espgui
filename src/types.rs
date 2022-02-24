pub mod chip {
    use clap::ArgEnum;
    use espmonitor::Chip as EspChip;

    #[derive(ArgEnum, Clone, Debug)]
    pub enum Chip {
        ESP32,
        ESP8266,
        ESP32C3,
    }

    impl Chip {
        fn as_str(&self) -> &'static str {
            match self {
                Chip::ESP32 => "esp32",
                Chip::ESP8266 => "esp8266",
                Chip::ESP32C3 => "esp32c3",
            }
        }

        pub fn to_esp_chip(&self) -> EspChip {
            EspChip::try_from(self.as_str()).unwrap_or_default()
        }
    }
}

pub mod serial {
    use std::fmt;

    use clap::ArgEnum;

    #[cfg(target_os = "windows")]
    #[derive(ArgEnum, Clone, Debug)]
    #[clap(rename_all = "verbatim")]
    pub enum Serial {
        COM0,
        COM1,
        COM2,
        COM3,
    }

    #[cfg(target_os = "linux")]
    #[derive(ArgEnum, Clone, Debug)]
    #[clap(rename_all = "verbatim")]
    #[allow(non_camel_case_types)]
    pub enum Serial {
        ttyUSB0,
        ttyUSB1,
        ttyUSB2,
        ttyUSB3,
    }

    // todo: macos ports
    #[cfg(target_os = "macos")]
    #[derive(ArgEnum, Clone, Debug)]
    #[clap(rename_all = "verbatim")]
    pub enum Serial {}

    impl Serial {
        fn as_str(&self) -> &'static str {
            #[cfg(target_os = "windows")]
            match self {
                Serial::COM0 => "COM0",
                Serial::COM1 => "COM1",
                Serial::COM2 => "COM2",
                Serial::COM3 => "COM3",
            }

            #[cfg(target_os = "linux")]
            match self {
                Serial::ttyUSB0 => "/dev/ttyUSB0",
                Serial::ttyUSB1 => "/dev/ttyUSB1",
                Serial::ttyUSB2 => "/dev/ttyUSB2",
                Serial::ttyUSB3 => "/dev/ttyUSB3",
            }

            #[cfg(target_os = "macos")]
            todo!("macos ports")
        }
    }

    impl fmt::Display for Serial {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }
}
