use std::sync::{Arc, Mutex};

use pest::{iterators::*, Parser};

use crate::blocks::amplifier::AmplifierBlock;
use crate::blocks::constant::ConstantBlock;
use crate::blocks::oscillator::OscillatorBlock;
use crate::blocks::sequencer::SequencerBlock;
use crate::blocks::stereo::StereoBlock;
use crate::blocks::{BlockType, SignalBlock, SignalSource};
use crate::error::HarmoniconError;
use crate::driver::HarmoniconDriver;
use crate::note::Note;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct HarmoniconParser;

fn parse_anon_init(pair: Pair<'_, Rule>, driver: &HarmoniconDriver) -> crate::Result<Arc<Mutex<dyn SignalBlock>>> {
    let mut inner = pair.into_inner();
    let block_type = inner.next().unwrap().as_str().parse().unwrap();
    let init = inner.next().unwrap();

    use BlockType::*;
    match block_type {
        Constant => parse_const_init(init).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
        Oscillator => parse_osc_init(init, driver).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
        Amplifier => parse_amp_init(init, driver).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
        Stereo => parse_stereo_init(init, driver).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
        Sequencer => parse_sequencer_init(init, driver).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
    }
}

fn parse_param_rhs(pair: Pair<'_, Rule>, driver: &HarmoniconDriver) -> crate::Result<SignalSource> {
    match pair.as_rule() {
        Rule::name => {
            driver.get_block(pair.as_str())
                .map(|b| SignalSource::Named(pair.as_str().to_owned(), Arc::downgrade(b)))
                .ok_or(HarmoniconError::UnknownBlock(pair.as_str().to_owned()))
        },
        Rule::anonymous => {
            parse_anon_init(pair, driver)
                .map(SignalSource::Anonymous)
        },
        _ => Err(HarmoniconError::TypeError("name or initializer", "other")),
    }
}

fn parse_sequence(pair: Pair<'_, Rule>) -> crate::Result<Vec<Note>> {
    if pair.as_rule() != Rule::sequence {
        return Err(HarmoniconError::TypeError("sequence", "other"));
    }

    let mut seq = Vec::new();
    for pair in pair.into_inner() {
        let note = str::parse(pair.as_str()).unwrap();
        seq.push(note);
    }

    Ok(seq)
}

fn parse_osc_init(pair: Pair<'_, Rule>, driver: &HarmoniconDriver) -> crate::Result<OscillatorBlock> {
    if pair.as_rule() != Rule::block_initializer {
        Err(HarmoniconError::TypeError("oscillator initializer", "other initializer"))
    } else {
        let mut osc = OscillatorBlock::default();
        for item in pair.into_inner() {
            let mut inner = item.into_inner();
            let key = inner.next().unwrap().as_str();

            let value = inner.next().unwrap();
            let rhs = parse_param_rhs(value, driver)?;

            match key {
                "frequency" | "freq" => osc.update_frequency(rhs),
                _ => return Err(HarmoniconError::UnknownProperty(key.to_owned(), "oscillator")),
            }
        }
        Ok(osc)
    }
}

fn parse_amp_init(pair: Pair<'_, Rule>, driver: &HarmoniconDriver) -> crate::Result<AmplifierBlock> {
    if pair.as_rule() != Rule::block_initializer {
        Err(HarmoniconError::TypeError("amplifier initializer", "other initializer"))
    } else {
        let mut amp = AmplifierBlock::default();
        for item in pair.into_inner() {
            let mut inner = item.into_inner();
            let key = inner.next().unwrap().as_str();

            let value = inner.next().unwrap();
            let rhs = parse_param_rhs(value, driver)?;

            match key {
                "source" | "src" => amp.update_source(rhs),
                "multiplicator" | "mult" => amp.update_multiplicator(rhs),
                _ => return Err(HarmoniconError::UnknownProperty(key.to_owned(), "amplifier")),
            }
        }
        Ok(amp)
    }
}

fn parse_stereo_init(pair: Pair<'_, Rule>, driver: &HarmoniconDriver) -> crate::Result<StereoBlock> {
    if pair.as_rule() != Rule::block_initializer {
        Err(HarmoniconError::TypeError("stereo initializer", "other initializer"))
    } else {
        let mut stereo = StereoBlock::default();
        for item in pair.into_inner() {
            let mut inner = item.into_inner();
            let key = inner.next().unwrap().as_str();

            let value = inner.next().unwrap();
            let rhs = parse_param_rhs(value, driver)?;

            match key {
                "left" | "l" => stereo.update_left(rhs),
                "right" | "r" => stereo.update_right(rhs),
                "shift" | "s" => stereo.update_shift(rhs),
                _ => return Err(HarmoniconError::UnknownProperty(key.to_owned(), "stereo")),
            }
        }
        Ok(stereo)
    }
}

fn parse_sequencer_init(pair: Pair<'_, Rule>, driver: &HarmoniconDriver) -> crate::Result<SequencerBlock> {
    if pair.as_rule() != Rule::block_initializer {
        Err(HarmoniconError::TypeError("stereo initializer", "other initializer"))
    } else {
        let mut sequencer = SequencerBlock::default();
        for item in pair.into_inner() {
            let mut inner = item.into_inner();
            let key = inner.next().unwrap().as_str();

            let value = inner.next().unwrap();
            if key == "seq" || key == "sequence" {
                let seq = parse_sequence(value)?;
                sequencer.update_sequence(seq);
            } else {
                let rhs = parse_param_rhs(value, driver)?;

                match key {
                    "bpm" => sequencer.update_bpm(rhs),
                    "spacing" => sequencer.update_spacing(rhs),
                    _ => return Err(HarmoniconError::UnknownProperty(key.to_owned(), "sequencer")),
                }
            }
        }
        Ok(sequencer)
    }
}

fn parse_const_init(pair: Pair<'_, Rule>) -> crate::Result<ConstantBlock> {
    if pair.as_rule() != Rule::const_initializer {
        Err(HarmoniconError::TypeError("constant initializer", "other initializer"))
    } else {
        let val = pair.as_str().parse().unwrap();
        Ok(ConstantBlock::new(val))
    }
}


pub fn parse_stage2(pair: Pair<'_, Rule>) -> crate::Result<HarmoniconDriver> {
    if pair.as_rule() != Rule::file {
        panic!("Unexpected rule {:?}", pair.as_rule());
    }

    let mut driver = HarmoniconDriver::new();

    for assgn_pair in pair.into_inner().filter(|p| p.as_rule() == Rule::assignment) {
        // Use unwrap() wherever soundness is already guaranteed by the grammar parser
        let mut inner = assgn_pair.into_inner();

        let type_str = inner.next().unwrap();
        let name = inner.next().unwrap().as_str();
        let initializer = inner.next().unwrap().into_inner().next().unwrap();

        use BlockType::*;
        match str::parse(type_str.as_str()).unwrap() {
            Constant => driver.register_block(name.to_owned(), parse_const_init(initializer)?),
            Oscillator => driver.register_block(name.to_owned(), parse_osc_init(initializer, &driver)?),
            Amplifier => driver.register_block(name.to_owned(), parse_amp_init(initializer, &driver)?),
            Stereo => driver.register_block(name.to_owned(), parse_stereo_init(initializer, &driver)?),
            Sequencer => driver.register_block(name.to_owned(), parse_sequencer_init(initializer, &driver)?),
        };
    }

    Ok(driver)
}

pub fn parse_stage1(input: &str) -> crate::Result<Pair<'_, Rule>> {
    Ok(HarmoniconParser::parse(Rule::file, input)?
        .next()
        .unwrap())
}
