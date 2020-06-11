use super::Platform;

/// A collection of tools for photoprintit software
#[derive(StructOpt, Debug)]
#[structopt(name = "photoprintit-tools")]
pub enum Cli {
    /// Prepare download
    #[structopt(name = "prepare")]
    Prepare(Prepare),
}

/// Prepare download
#[derive(StructOpt, Debug)]
pub struct Prepare {
    /// KEYACCID
    #[structopt(short = "k", long = "keyaccid")]
    pub keyaccid: String,
    /// (full) Locale
    #[structopt(short = "l", long = "locale")]
    pub locale: String,
    /// Client ID
    #[structopt(short = "c", long = "client-id")]
    pub client_id: String,
    /// HPS version
    #[structopt(short = "h", long = "hps-version")]
    pub hps_version: String,
    /// Platform (a, l, l64, m, w, w64)
    #[structopt(short = "p", long = "platform", parse(from_str))]
    pub platform: Platform,
    /// Imply platform `a` (any), include complete set of resources for install
    #[structopt(short = "a", long = "all")]
    pub all: bool,
    /// Downlaod server
    #[structopt(
        short = "d",
        long = "download-server",
        default_value = "https://dls.photoprintit.com"
    )]
    pub dl_server: String,
}
