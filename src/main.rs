use clap::Parser;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};


use std::path::PathBuf;
use std::time::Duration;
use std::{fs, process, sync::*};

use crate::blocks::amplifier::AmplifierBlock;
use crate::blocks::constant::ConstantBlock;
use crate::blocks::oscillator::OscillatorBlock;
use crate::driver::HarmoniconDriver;
use crate::error::resolve;


mod blocks;
mod error;
mod driver;
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
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    sink.append(driver);
    sink.sleep_until_end();
}
