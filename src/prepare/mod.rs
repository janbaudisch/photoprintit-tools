mod parser;

use super::cli::Prepare;
use super::Platform;

pub fn main(cli: &Prepare) {
    let index_url = format!(
        "{}/download/Data/{}-{}/hps/{}-index-{}.txt",
        cli.dl_server, cli.keyaccid, cli.locale, cli.client_id, cli.hps_version
    );

    debug!("index URL: {}", index_url);

    let index = reqwest::get(&index_url)
        .expect("GET index failed!")
        .text()
        .expect("Reading index failed!");

    for line in index.lines() {
        debug!("index line: {}", line);
        let line = format!("{};", line);
        let (_, file) = parser::file(&line).expect("Parsing index failed!");

        if file.platform == cli.platform || (file.platform == Platform::Any && cli.all) {
            println!("{}/{}", cli.dl_server, file);
        }
    }
}
