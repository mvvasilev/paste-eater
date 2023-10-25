use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    /// Where to store the pastes. By default this is a "data" sub-directory within the system-specific configuration directory for the application ( see https://github.com/dirs-dev/directories-rs#basedirs" ).
    #[arg(short, long)]
    pub location: Option<String>,

    /// Max size of a single paste
    #[arg(short = 'f', long, default_value_t = 10000000)]
    pub max_file_size: u64,

    // /// Max size of all pastes existing on the server. Once this limit is reached, pastes will be deleted, starting with oldest first.
    // #[arg(short = 's', long, default_value_t = 100000000)]
    // pub max_storage_size: u64,

    /// Constant size of new paste identifiers being generated. Changing this does not alter existing pastes.
    #[arg(short, long, default_value_t = 12)]
    pub name_size: usize,

    #[arg(short, long, default_value_t = true)]
    pub compress: bool
}