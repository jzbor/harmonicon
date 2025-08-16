use clap::Parser;


use std::path::PathBuf;

use crate::driver::HarmoniconDriver;
use crate::error::resolve;


mod blocks;
mod error;
mod driver;
mod note;
mod parse;
mod reload;

const SAMPLE_RATE: u32 = 44100;

type Result<T> = error::HarmoniconResult<T>;
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}


fn main() {
    let args = Args::parse();

    let rx = reload::start_reload_thread(args.file.clone());
    let mut driver = resolve(HarmoniconDriver::parse_from_file(&args.file));
    driver.set_update_rx(rx);

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    sink.append(driver);
    sink.sleep_until_end();
}
