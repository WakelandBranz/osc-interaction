// thank you https://gitlab.com/marcempunkt/config-toml

// in the future I would like to figure out how to handle errors like this more gracefully.
// i feel like these are handled in an improper manner

use std::fs;
use std::io::Error;
use serde::{ Serialize, Deserialize };
use toml;

use log::{debug, info, error};

use crate::utils::cli::{self, Benchmark};

// this method of parsing data seems convoluted.  if anyone has tips on how to implement this
// in a better way, please let me know.

const CONFIG_FILEPATHS: [&str ; 2] = [
    "config\\config.toml",
    "config\\Config.toml"
];

const DEFAULT_NETWORK_LOCALHOST: &str = "127.0.0.1";
const DEFAULT_NETWORK_RX_PORT: u16 = 9001;
const DEFAULT_NETWORK_TX_PORT: u16 = 9000;
const DEFAULT_LOCAL_TIME: bool = false;
const DEFAULT_SPOTIFY: bool = false;
const DEFAULT_DEBUG: bool = false;

const LOCAL_TIME_LENGTH: usize = 8;


#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigNetwork {
    pub localhost: Option<String>,
    pub rx_port: Option<u16>,
    pub tx_port: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFeatures {
    pub text: Option<String>,
    pub local_time: Option<bool>,
    pub spotify: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigDev {
    pub debug: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub network: Option<ConfigNetwork>,
    pub features: Option<ConfigFeatures>,
    pub dev: Option<ConfigDev>,
}

impl Config {
    pub fn new() -> Self {
        let benchmark = cli::Benchmark::new();

        let mut content: String = "".to_string();

        for filepath in CONFIG_FILEPATHS {
            let result: Result<String, Error> = fs::read_to_string(filepath);

            match result {
                Ok(data) => {
                    content = data;
                    debug!("Successfully read from file: {}", filepath);
                    break;
                }
                Err(e) => {
                    error!("Failed to read from file {}: {}", filepath, e)
                }
            }
        }

        let config: Config = toml::from_str(&content).unwrap_or_else(|_| {
            // file got messed up, return default
            error!("Failed to create Config Object out of config file.");
            error!("Couldn't read config.toml file. Check for file validity.");
            Config::default()
        });

        let (localhost, rx_port, tx_port) = match config.network {
            Some(network) => {
                let network_localhost: String = network.localhost.unwrap_or_else(|| {
                    error!("Failed to read network -> localhost from config | Defaulting to '{}'", DEFAULT_NETWORK_LOCALHOST);
                    DEFAULT_NETWORK_LOCALHOST.to_string()
                });

                let network_rx_port: u16 = network.rx_port.unwrap_or_else(|| {
                    error!("Failed to read network -> rx_port from config | Defaulting to {}", DEFAULT_NETWORK_RX_PORT);
                    DEFAULT_NETWORK_RX_PORT
                });

                let network_tx_port: u16 = network.tx_port.unwrap_or_else(|| {
                    error!("Failed to read network -> tx_port from config | Defaulting to {}", DEFAULT_NETWORK_TX_PORT);
                    DEFAULT_NETWORK_TX_PORT
                });

                (network_localhost, network_rx_port, network_tx_port)
            },
            None => {
                error!("Missing table from config: network | Defaulting to '127.0.0.1', 9001, and 9000");
                (DEFAULT_NETWORK_LOCALHOST.to_string(), DEFAULT_NETWORK_RX_PORT, DEFAULT_NETWORK_TX_PORT)
            }

        };

        /*
        FEATURES
        */
        
        let (text, local_time, spotify) = match config.features {
            Some(features) => {
                let features_text: String = features.text.unwrap_or_else(|| {
                    error!("Failed to read local_time -> text from config | Defaulting to ''");
                    "".to_string()
                });

                let features_local_time: bool = features.local_time.unwrap_or_else(|| {
                    error!("Failed to read local_time -> local_time from config | Defaulting to false");
                    false
                });

                let features_spotify: bool = features.spotify.unwrap_or_else(|| {
                    error!("Failed to read features -> spotify from config | Defaulting to false");
                    false
                });

                (features_text, features_local_time, features_spotify)
            }
            None => {
                error!("Missing table from config: features | Defaulting to false");
                ("".to_string(), DEFAULT_LOCAL_TIME, DEFAULT_SPOTIFY)
            }
        };

        let (debug) = match config.dev {
            Some(dev) => {
                let dev_debug: bool = dev.debug.unwrap_or_else(|| {
                    error!("Failed to read dev -> debug from config | Defaulting to false");
                    false
                });

                (dev_debug)
            }
            None => {
                error!("Missing table from config: dev | Defaulting to false");
                (DEFAULT_DEBUG)
            }
        };

        debug!("Parsed all available info from config: {:?}", benchmark.get_elapsed());

        Config {
            network: Some(ConfigNetwork {
                localhost: Some(localhost),
                rx_port: Some(rx_port),
                tx_port: Some(tx_port),
            }),
            features: Some(ConfigFeatures {
                text: Some(text),
                local_time: Some(local_time),
                spotify: Some(spotify),
            }),
            dev: Some(ConfigDev {
                debug: Some(debug),
            }),
        }
    }

    pub fn update_features(&mut self) {
        let benchmark = cli::Benchmark::new();

        debug!("Reparsing features info...");

        let mut content: String = "".to_string();

        for filepath in CONFIG_FILEPATHS {
            let result: Result<String, Error> = fs::read_to_string(filepath);

            match result {
                Ok(data) => {
                    content = data;
                    debug!("Successfully read from file: {}", filepath);
                    break;
                }
                Err(e) => {
                    error!("Failed to read from file {}: {}", filepath, e)
                }
            }
        }

        let config: Config = toml::from_str(&content).unwrap_or_else(|_| {
            // file got messed up, return default
            error!("Failed to create Config Object out of config file.");
            error!("Couldn't read config.toml file. Check for file validity. Setting all config values to default.");
            Config::default() 
        });

        let (text, local_time, spotify) = match config.features {
            Some(features) => {
                let features_text: String = features.text.unwrap_or_else(|| {
                    error!("Failed to read local_time -> text from config | Defaulting to ''");
                    "".to_string()
                });

                let features_local_time: bool = features.local_time.unwrap_or_else(|| {
                    error!("Failed to read local_time -> local_time from config | Defaulting to false");
                    false
                });

                let features_spotify: bool = features.spotify.unwrap_or_else(|| {
                    error!("Failed to read features -> spotify from config | Defaulting to false");
                    false
                });

                (features_text, features_local_time, features_spotify)
            }
            None => {
                error!("Missing table from config: features | Defaulting to false");
                ("".to_string(), DEFAULT_LOCAL_TIME, DEFAULT_SPOTIFY)
            }
        };

        self.features = Some(ConfigFeatures {
            text: Some(text),
            local_time: Some(local_time),
            spotify: Some(spotify),
        });

        info!("Features reparsed and updated! Time elapsed: {:?}", benchmark.get_elapsed());
    }
}

impl Default for ConfigNetwork {
    fn default() -> Self {
        ConfigNetwork {
            localhost: Some("127.0.0.1".to_string()),
            rx_port: Some(9001),
            tx_port: Some(9000),
        }
    }
}

impl Default for ConfigDev {
    fn default() -> Self {
        ConfigDev {
            debug: Some(false),
        }
    }
}

impl Default for ConfigFeatures {
    fn default() -> Self {
        ConfigFeatures {
            text: Some("Placeholder text, no input detected in config.toml".to_string()),
            local_time: Some(false),
            spotify: Some(false),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            network: Some(ConfigNetwork::default()),
            features: Some(ConfigFeatures::default()),
            dev: Some(ConfigDev::default()),
        }
    }
}