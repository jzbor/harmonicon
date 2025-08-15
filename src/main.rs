use clap::Parser;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};


use std::time::Duration;
use std::{fs, process, sync::*};

use crate::blocks::amplifier::AmplifierBlock;
use crate::blocks::constant::ConstantBlock;
use crate::blocks::oscillator::OscillatorBlock;
use crate::mixer::HarmoniconMixer as HarmoniconMixer;


mod error;
mod parse;
mod blocks;
mod mixer;

const SAMPLE_RATE: u32 = 44100;

type Result<T> = error::HarmoniconResult<T>;
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
}


fn main() {
    let args = Args::parse();

    let content = fs::read_to_string("./test.hc").unwrap();
    let test1 = match parse::parse_stage1(&content) {
        Ok(t1) => t1,
        Err(e) => { eprintln!("{}", e); process::exit(1) },
    };
    let mixer = parse::parse_stage2(test1)
        .map_err(|e| e.to_string())
        .unwrap();

    // let mut mixer = HarmoniconMixer::new();
    // let c = ConstantBlock::new(440.0);
    // let c = mixer.register_block("c".to_owned(), c);
    // let d = ConstantBlock::new(1.0);
    // let d = mixer.register_block("d".to_owned(), d);
    // let osc = OscillatorBlock::new(Arc::downgrade(&d));
    // let osc = mixer.register_block("osc".to_owned(), osc);
    // let amp = AmplifierBlock::new(Arc::downgrade(&osc), 440.0);
    // let amp = mixer.register_block("amp".to_owned(), amp);
    // let out = OscillatorBlock::new(Arc::downgrade(&amp));
    // let out = mixer.register_block("out".to_owned(), out);


    // _stream must live as long as the sink
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // Add a dummy source of the sake of the example.
    sink.append(mixer);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
