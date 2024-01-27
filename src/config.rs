// thank you https://gitlab.com/marcempunkt/config-toml

// in the future I would like to figure out how to handle errors like this more gracefully.
// i feel like these are handled in an improper manner

use std::fs;
use std::io::Error;
use serde::{ Serialize, Deserialize };
use toml;

use log::{debug, info, error};

 

// this method of parsing data seems convoluted.  if anyone has tips on how to implement this
// in a better way, please let me know.

const DEFAULT_NETWORK_LOCALHOST: &str = "127.0.0.1";
const DEFAULT_NETWORK_RX_PORT: u16 = 9001;
const DEFAULT_NETWORK_TX_PORT: u16 = 9000;
const DEFAULT_SPOTIFY: bool = false;
const DEFAULT_DEBUG: bool = false;


#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigNetwork {
    pub localhost: Option<String>,
    pub rx_port: Option<u16>,
    pub tx_port: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFeatures {
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
        let config_filepaths: [&str ; 2] = [
            "config\\config.toml",
            "config\\Config.toml"
        ];

        let mut content: String = "".to_string();

        for filepath in config_filepaths {
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
            error!("Failed to create Config Object out of config file.");
            panic!("Couldn't read config.toml file. Check for file validity.")
        });

        debug!("Created Config Object out of config file");


        // future goal: implement these variables so that they are parsed into their respective struct

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

        let (spotify) = match config.features {
            Some(features) => {
                let features_spotify: bool = features.spotify.unwrap_or_else(|| {
                    error!("Failed to read features -> spotify from config | Defaulting to false");
                    false
                });

                (features_spotify)
            }
            None => {
                error!("Missing table from config: features | Defaulting to false");
                (DEFAULT_SPOTIFY)
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

        info!("Available data parsed from config");

        Config {
            network: Some(ConfigNetwork {
                localhost: Some(localhost),
                rx_port: Some(rx_port),
                tx_port: Some(tx_port),
            }),
            features: Some(ConfigFeatures {
                spotify: Some(spotify),
            }),
            dev: Some(ConfigDev {
                debug: Some(debug),
            }),
        }
    }
}