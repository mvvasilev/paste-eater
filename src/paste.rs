use std::{path::{Path, PathBuf}, fs::File};

use chrono::{DateTime, Local};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rand::{Rng, distributions::Alphanumeric};
use serde_derive::{Serialize, Deserialize};

use crate::{config::ConfigurationHandler, error::PasteEaterError};

type PasteError = PasteEaterError;

type PasteUID = String;

const LANGUAGES: &[&str] = &[
    "abap",
    "apex",
    "azcli",
    "bat",
    "bicep",
    "cameligo",
    "clojure",
    "coffee",
    "cpp",
    "csharp",
    "csp",
    "css",
    "cypher",
    "dart",
    "dockerfile",
    "ecl",
    "elixir",
    "flow9",
    "freemarker2",
    "fsharp",
    "go",
    "graphql",
    "handlebars",
    "hcl",
    "html",
    "ini",
    "java",
    "javascript",
    "json",
    "julia",
    "kotlin",
    "less",
    "lexon",
    "liquid",
    "lua",
    "m3",
    "markdown",
    "mdx",
    "mips",
    "msdax",
    "mysql",
    "objective-c",
    "pascal",
    "pascaligo",
    "perl",
    "pgsql",
    "php",
    "pla",
    "postiats",
    "powerquery",
    "powershell",
    "protobuf",
    "pug",
    "python",
    "qsharp",
    "r",
    "razor",
    "redis",
    "redshift",
    "restructuredtext",
    "ruby",
    "rust",
    "sb",
    "scala",
    "scheme",
    "scss",
    "shell",
    "solidity",
    "sophia",
    "sparql",
    "sql",
    "st",
    "swift",
    "systemverilog",
    "tcl",
    "twig",
    "typescript",
    "vb",
    "wgsl",
    "xml",
    "yaml"
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasteOutput {
    pub encrypted: bool,
    pub created: String,
    pub last_accessed: String, 
    pub language: String,
    pub data: String
}

pub struct PasteHandler {
    config_handler: ConfigurationHandler,
    // storage_size_cache: u64, TODO: Optimize storage size tracking

}

impl PasteHandler {
    pub fn new(config_handler: ConfigurationHandler) -> Self {
        Self {
            config_handler,
            // storage_size_cache: 0
        }
    }

    pub fn create_new_paste(&self, encrypt: bool, language: &String, paste_data: &String) -> Result<PasteUID, PasteError> {
        let config = self.config_handler.fetch_config()?;

        let data_path = Path::new(&config.files_location);

        match data_path.try_exists() {
            Ok(exists) => {
                if !exists {
                    match std::fs::create_dir_all(data_path) {
                        Ok(_) => {},
                        Err(internal) => return Err(PasteError::new_internal(&format!("Failed to create data directory '{}'.", data_path.display()), Box::new(internal))),
                    }
                }
            },
            Err(err) => return Err(PasteError::new_internal(&format!("Failed to check if data directory '{}' exists.", data_path.display()), Box::new(err))),
        }

        // TODO: Encrypt (?)

        if paste_data.len() as u64 > config.files_max_single_file_size_in_bytes {
            return Err(PasteError::new("Uploaded paste is larger than maximum allowed size."));
        }


        // TODO: Enforce max size for single file and max total sizes. Note: scrapped for now.
        // if self.determine_current_storage_size() + paste_data.len() as u64 > config.files_max_total_file_size_in_bytes {
        //     if let Err(e) = std::fs::remove_file(self.find_oldest_paste_larger_than(paste_data.len() as u64)) {
        //         return Err(PasteError::new_internal("Unable to save paste, overloads capacity and no additional storage could be acquired.", Box::new(e)));
        //     }
        // }

        let (file_path, uid) = self.create_new_paste_file(Path::new(&config.files_location), config.name_size)?;

        // paste file format:
        // 0: is encrypted | is compressed | unused | unused | unused | unused | unused | unused
        // 1: language byte
        // 2-end: data bytes

        let mut file_data: Vec<u8> = Vec::new();

        let mut flags = 0b0000_0000;

        if encrypt {
            flags |= 0b1000_0000;
        }

        if config.compress {
            flags |= 0b0100_0000;
        }

        file_data.push(flags);

        let language_index = LANGUAGES.iter().position(|&r| r == language).unwrap_or(0);

        file_data.push(language_index as u8);

        if config.compress {
            file_data.extend(compress_prepend_size(paste_data.as_bytes()));
        }

        match std::fs::write(&file_path, file_data) {
            Ok(_) => Ok(uid),
            Err(e) => Err(PasteError::new_internal(&format!("Failed to write paste file '{}'.", file_path.display()), Box::new(e))),
        }
    }

    // fn determine_current_storage_size(&self) -> u64 {
    //     0
    // }

    // fn find_oldest_paste_larger_than(&self, size: u64) -> PathBuf {
    //     Path::new(".").to_path_buf()
    // }

    fn create_new_paste_file(&self, directory: &Path, name_size: usize) -> Result<(PathBuf, PasteUID), PasteError> {
        let mut uid: String;
        let mut file_path;

        loop {
            uid = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(name_size)
                .map(char::from)
                .collect();

            file_path = directory.to_path_buf();

            file_path.push(Path::new(&format!("{}.paste", uid)));

            if !file_path.exists() {
                break;
            }
        }

        match File::create(&file_path) {
            Ok(_) => Ok((file_path, uid)),
            Err(e) => Err(PasteError::new_internal(&format!("Failed to create paste file '{}'.", file_path.display()), Box::new(e))),
        }
    }

    pub fn fetch_raw_paste(&self, uid: PasteUID) -> Result<PasteOutput, PasteError> {
        let config = self.config_handler.fetch_config()?;

        let mut file_path = Path::new(&config.files_location).to_path_buf();

        file_path.push(Path::new(&format!("{}.paste", uid)));

        if !file_path.exists() {
            return Err(PasteError::new(&format!("Requested paste '{}' does not exist.", uid)));
        }

        let file_bytes = match std::fs::read(&file_path) {
            Ok(bytes) => bytes,
            Err(e) => return Err(PasteError::new_internal(&format!("Failed to read paste '{}'.", uid), Box::new(e))),
        };

        let flags = file_bytes[0];

        let encrypted = 0b1000_0000 & flags != 0;
        let compressed = 0b0100_0000 & flags != 0;

        let language = file_bytes[1];

        let bytes = if compressed {
            match decompress_size_prepended(&file_bytes[2..]) {
                Ok(d) => d,
                Err(e) => return Err(PasteError::new_internal(&format!("Failed to decompress paste '{}'.", uid), Box::new(e))),
            }
        } else {
            file_bytes[2..].to_vec()
        };

        let data = match String::from_utf8(bytes) {
                Ok(d) => d,
                Err(e) => return Err(PasteError::new_internal(&format!("Failed to parse paste '{}'.", uid), Box::new(e))),
            };

        let mut paste = PasteOutput {
            encrypted,
            last_accessed: DateTime::UNIX_EPOCH.to_rfc2822(),
            created: DateTime::UNIX_EPOCH.to_rfc2822(),
            language: LANGUAGES[language as usize].to_string(),
            data
        };

        if let Ok(metadata) = std::fs::metadata(file_path) {
            paste.created = match metadata.created() {
                Ok(created) => DateTime::<Local>::from(created).to_rfc2822(),
                Err(_) => paste.created,
            };

            paste.last_accessed = match metadata.accessed() {
                Ok(accessed) => DateTime::<Local>::from(accessed).to_rfc2822(),
                Err(_) => paste.last_accessed,
            };
        }

        Ok(paste)
    }

    pub fn delete_paste(&self, uid: PasteUID) -> Result<(), PasteError> {
        let config = self.config_handler.fetch_config()?;

        let mut file_path = Path::new(&config.files_location).to_path_buf();

        file_path.push(Path::new(&format!("{}.paste", uid)));

        match std::fs::remove_file(file_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(PasteError::new_internal(&format!("Failed to delete paste '{}'.", uid), Box::new(e))),
        }
    }
}

