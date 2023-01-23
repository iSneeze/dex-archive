use clap::{Parser, Subcommand, Args};

/// A program to automatically check for new vtuber streams on youtube using the holodex api, which then starts a yt-dlp process to archive them
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct YtVtArgs {

    #[clap(subcommand)]
    pub subcommand: Command, 

}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// add, list and remove channels to monitor
    Channel(ChannelCommand),
    /// configure holodex and yt-dlp
    Config(ConfigCommand),
    /// start monitoring
    Run
}

#[derive(Debug, Args)]
pub struct ConfigCommand{
    /// Cookies file to download membership streams
    #[arg(short = 'f', long)]
    pub cookies_file: Option<String>,

    /// Take cookies from this browser to download membership streams
    #[arg(short = 'b', long)]
    pub cookies_browser: Option<String>,

    /// Holodex API Key
    #[arg(short = 'a', long,)]
    pub holodex_api_key: Option<String>,

    /// Number of threads for yt-dlp per process
    #[arg(long, default_value_t = 2)]
    pub ytdlp_download_threads: u8,

    /// yt-dlp title string 
    #[arg(short  = 't', long)]
    pub ytdlp_title: Option<String>,

    /// Video download resolution
    #[arg(short  = 'r',long)]
    pub ytdlp_res: Option<String>,

    /// Holodex API poll interval in seconds
    #[arg(short  = 'i', long, default_value_t = 1800)]
    pub holodex_poll_interval: u16,
}
#[derive(Debug, Args)]
pub struct ChannelCommand {
    #[clap(subcommand)]
    pub command: ChannelSubCommand
}

#[derive(Debug, Subcommand)]
pub enum ChannelSubCommand{
    /// add a new channel by providing an organisation and a channel name
    Add(AddChannelCommand),
    /// list all added channels
    List,
    /// remove a channel
    Remove,
}

#[derive(Debug, Args)]
pub struct AddChannelCommand{
    /// organisation (Hololive, Nijisanji, idol corp, Independents, etc.)
    #[arg(short = 'o', long)]
    pub organisation: String,
    /// channel name search string
    #[arg(short = 'c', long)]
    pub channel_name: String
}
