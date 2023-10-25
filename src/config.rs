use std::{path::{Path, PathBuf}, fs::File, io::Write};

use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};

use crate::{args::Args, error::PasteEaterError};

const PASTE_EATER_CONFIG_FILE: &str = "config.toml";
const PASTE_EATER_QUALIFIER: &str = "dev.mvvasilev";
const PASTE_EATER_ORGANIZATION: &str = "mvvasilev";
const PASTE_EATER_APPLICATION: &str = "paste-eater";

type ConfigurationError = PasteEaterError;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PasteEaterConfig {
    /// Where to store the pastes
    #[serde(default = "default_files_location")]
    pub files_location: String,

    /// Max size of a single paste
    #[serde(default = "default_files_max_single_file_size_in_bytes")]
    pub files_max_single_file_size_in_bytes: u64,

    // /// Max size of all pastes existing on the server. Once this limit is reached, pastes will be deleted, starting with oldest first.
    // #[serde(default = "default_files_max_total_file_size_in_bytes")]
    // pub files_max_total_file_size_in_bytes: u64,

    /// Constant size of new paste identifiers being generated. Changing this should not alter existing pastes.
    #[serde(default = "default_name_size")]
    pub name_size: usize,

    #[serde(default = "default_compress")]
    pub compress: bool

    // TODO: Encrypted pastes?
}

fn default_files_location() -> String {
    if let Some(config_dirs) = ProjectDirs::from(PASTE_EATER_QUALIFIER, PASTE_EATER_ORGANIZATION, PASTE_EATER_APPLICATION) {
        let mut dir = config_dirs.config_dir().to_path_buf();

        dir.push(Path::new("data"));

        dir.display().to_string()
    } else {
        Path::new(".").to_path_buf().display().to_string()
    }
}

fn default_files_max_single_file_size_in_bytes() -> u64 {
    10_000_000
}

fn default_files_max_total_file_size_in_bytes() -> u64 {
    100_000_000
}

fn default_name_size() -> usize {
    12
}

fn default_compress() -> bool {
    true
}

pub struct ConfigurationHandler {
    config_path: PathBuf
}

impl ConfigurationHandler {
    pub fn new() -> Result<Self, ConfigurationError> {
        if let Some(config_dirs) = ProjectDirs::from(PASTE_EATER_QUALIFIER, PASTE_EATER_ORGANIZATION, PASTE_EATER_APPLICATION) {
            ConfigurationHandler::new_with_path(config_dirs.config_dir())
        } else {
            Err(ConfigurationError::new("Unable to determine configuration path"))
        }
    }

    pub fn new_with_args(args: &Args) -> Result<Self, ConfigurationError> {
        let config = PasteEaterConfig {
            files_location: args.location.clone().unwrap_or(default_files_location()),
            files_max_single_file_size_in_bytes: args.max_file_size,
            // files_max_total_file_size_in_bytes: args.max_storage_size,
            name_size: args.name_size,
            compress: args.compress
        };

        

        if let Some(config_dirs) = ProjectDirs::from(PASTE_EATER_QUALIFIER, PASTE_EATER_ORGANIZATION, PASTE_EATER_APPLICATION) {
            ConfigurationHandler::new_with_defaults(config_dirs.config_dir(), &config)
        } else {
            Err(ConfigurationError::new("Unable to determine configuration path"))
        }
    }

    pub fn new_with_path(path: &Path) -> Result<Self, ConfigurationError> {
        ConfigurationHandler::new_with_defaults(path, &PasteEaterConfig {
            files_location: default_files_location(),
            files_max_single_file_size_in_bytes: default_files_max_single_file_size_in_bytes(),
            // files_max_total_file_size_in_bytes: default_files_max_total_file_size_in_bytes(),
            name_size: default_name_size(),
            compress: default_compress()
        })
    }

    pub fn new_with_defaults(path: &Path, default_config: &PasteEaterConfig) -> Result<Self, ConfigurationError> {
        if path.extension().is_some() {
            return Err(ConfigurationError::new(&format!("Provided configuration path '{}' is not a directory.", path.display())));
        }

        let mut pathbuf = path.to_path_buf();

        match pathbuf.try_exists() {
            Ok(exists) => {
                if !exists {
                    match std::fs::create_dir_all(&pathbuf) {
                        Ok(_) => {},
                        Err(internal) => return Err(ConfigurationError::new_internal(&format!("Failed to create configuration directory '{}'.", path.display()), Box::new(internal))),
                    }
                }
            },
            Err(err) => return Err(ConfigurationError::new_internal(&format!("Failed to check if configuration directory '{}' exists.", path.display()), Box::new(err))),
        }

        pathbuf.push(Path::new(PASTE_EATER_CONFIG_FILE));

        if !pathbuf.exists() {
            match File::create(&pathbuf) {
                Ok(mut f) => {

                    let config = PasteEaterConfig {
                        files_location: if default_config.files_location.is_empty() { default_files_location() } else { default_config.files_location.clone() },
                        files_max_single_file_size_in_bytes: default_config.files_max_single_file_size_in_bytes,
                        // files_max_total_file_size_in_bytes: default_config.files_max_total_file_size_in_bytes,
                        name_size: default_config.name_size,
                        compress: default_config.compress
                    };

                    let serialized_default_config = toml::to_string_pretty(&config);

                    let write_default_config = match serialized_default_config {
                        Ok(ser) => f.write_all(ser.as_bytes()),
                        Err(err) => return Err(ConfigurationError::new_internal("Failed to create default configuration.", Box::new(err))),
                    };

                    match write_default_config {
                        Ok(_) => {},
                        Err(err) => return Err(ConfigurationError::new_internal("Failed to create default configuration.", Box::new(err))),
                    }
                },
                Err(err) => return Err(ConfigurationError::new_internal(&format!("Failed to create configuration file '{}'", path.display()), Box::new(err))),
            }
        }

        Ok(Self {
            config_path: pathbuf
        })
    }

    pub fn fetch_config(&self) -> Result<PasteEaterConfig, ConfigurationError> {
        let config_as_string = std::fs::read_to_string(&self.config_path);

        let config: PasteEaterConfig = match config_as_string {
            Ok(s) => {
                match toml::from_str(&s) {
                    Ok(config) => config,
                    Err(err) => return Err(ConfigurationError::new_internal(&format!("Failed to parse configuration file '{}'", self.config_path.display()), Box::new(err))),
                }
            },
            Err(err) => return Err(ConfigurationError::new_internal(&format!("Failed to read configuration file '{}'", self.config_path.display()), Box::new(err))),
        };

        Ok(config)
    }
}