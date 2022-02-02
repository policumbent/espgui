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
        fn as_str(&self) -> &str {
            match self {
                Chip::ESP32 => "esp32",
                Chip::ESP8266 => "esp8266",
                Chip::ESP32C3 => "esp32c3"
            }
        }

        pub fn to_esp_chip(&self) -> EspChip {
            EspChip::try_from(self.as_str()).unwrap_or_default()
        }
    }
}

pub mod serial {
    use std::str::FromStr;
    use std::string::ParseError;

    use clap::ArgEnum;

// todo: add support for windows/mac/linux ports

    #[derive(ArgEnum, Clone, Debug)]
    pub enum Serial {
        Port1,
        Port2,
        Port3,
    }

    impl FromStr for Serial {
        type Err = ParseError;

        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            todo!()
        }
    }

    impl Serial {
        pub fn to_string(&self) -> String {
            unimplemented!()
        }
    }
}
