use std::str::FromStr;
use std::sync::{Arc, Mutex};

use pest::{iterators::*, Parser};

use crate::blocks::amplifier::AmplifierBlock;
use crate::blocks::constant::ConstantBlock;
use crate::blocks::oscillator::OscillatorBlock;
use crate::blocks::{SignalBlock, SignalSource};
use crate::error::HarmoniconError;
use crate::mixer::HarmoniconMixer;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct HarmoniconParser;

#[derive(Copy, Clone, Debug)]
enum HarmoniconType {
    Constant,
    Oscillator,
    Amplifier,
}

impl FromStr for HarmoniconType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HarmoniconType::*;
        match s {
            "constant" | "const" => Ok(Constant),
            "oscillator" | "osc" => Ok(Oscillator),
            "amplifier" | "amp" => Ok(Amplifier),
            _ => Err(()),
        }
    }
}

fn parse_anon_init(pair: Pair<'_, Rule>, mixer: &HarmoniconMixer) -> crate::Result<Arc<Mutex<dyn SignalBlock>>> {
    let mut inner = pair.into_inner();
    let block_type = inner.next().unwrap().as_str().parse().unwrap();
    let init = inner.next().unwrap();

    use HarmoniconType::*;
    match block_type {
        Constant => parse_const_init(init).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
        Oscillator => parse_osc_init(init, mixer).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
        Amplifier => parse_amp_init(init, mixer).map(|sb| Arc::new(Mutex::new(sb)) as Arc<Mutex<dyn SignalBlock>>),
    }
}

fn parse_param_rhs(pair: Pair<'_, Rule>, mixer: &HarmoniconMixer) -> crate::Result<SignalSource> {
    match pair.as_rule() {
        Rule::name => {
            mixer.get_block(pair.as_str())
                .map(|b| SignalSource::Named(pair.as_str().to_owned(), Arc::downgrade(b)))
                .ok_or(HarmoniconError::UnknownBlock(pair.as_str().to_owned()))
        },
        Rule::anonymous => {
            parse_anon_init(pair, &mixer)
                .map(|b| SignalSource::Anonymous(b))
        },
        _ => panic!(),
    }
}

fn parse_osc_init(pair: Pair<'_, Rule>, mixer: &HarmoniconMixer) -> crate::Result<OscillatorBlock> {
    if pair.as_rule() != Rule::block_initializer {
        Err(HarmoniconError::TypeError("oscillator initializer", "other initializer"))
    } else {
        let mut osc = OscillatorBlock::default();
        for item in pair.into_inner() {
            let mut inner = item.into_inner();
            let key = inner.next().unwrap().as_str();

            let value = inner.next().unwrap();
            let rhs = parse_param_rhs(value, mixer)?;

            match key {
                "frequency" | "freq" => osc.update_frequency(rhs),
                _ => return Err(HarmoniconError::UnknownProperty(key.to_owned(), "oscillator")),
            }
        }
        Ok(osc)
    }
}

fn parse_amp_init(pair: Pair<'_, Rule>, mixer: &HarmoniconMixer) -> crate::Result<AmplifierBlock> {
    if pair.as_rule() != Rule::block_initializer {
        Err(HarmoniconError::TypeError("oscillator initializer", "other initializer"))
    } else {
        let mut amp = AmplifierBlock::default();
        for item in pair.into_inner() {
            let mut inner = item.into_inner();
            let key = inner.next().unwrap().as_str();

            let value = inner.next().unwrap();
            let rhs = parse_param_rhs(value, mixer)?;

            match key {
                "source" | "src" => amp.update_source(rhs),
                "multiplicator" | "mult" => amp.update_multiplicator(rhs),
                _ => return Err(HarmoniconError::UnknownProperty(key.to_owned(), "amplifier")),
            }
        }
        Ok(amp)
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


pub fn parse_stage2(pair: Pair<'_, Rule>) -> crate::Result<HarmoniconMixer> {
    if pair.as_rule() != Rule::file {
        panic!("Unexpected rule {:?}", pair.as_rule());
    }

    let mut mixer = HarmoniconMixer::new();

    for assgn_pair in pair.into_inner().filter(|p| p.as_rule() == Rule::assignment) {
        // Use unwrap() wherever soundness is already guaranteed by the grammar parser
        let mut inner = assgn_pair.into_inner();

        let type_str = inner.next().unwrap();
        let name = inner.next().unwrap().as_str();
        let initializer = inner.next().unwrap().into_inner().next().unwrap();

        use HarmoniconType::*;
        match str::parse(type_str.as_str()).unwrap() {
            Constant => mixer.register_block(name.to_owned(), parse_const_init(initializer)?),
            Oscillator => mixer.register_block(name.to_owned(), parse_osc_init(initializer, &mixer)?),
            Amplifier => mixer.register_block(name.to_owned(), parse_amp_init(initializer, &mixer)?),
        };
    }

    Ok(mixer)
}

pub fn parse_stage1(input: &str) -> crate::Result<Pair<'_, Rule>> {
    Ok(HarmoniconParser::parse(Rule::file, input)?
        .next()
        .unwrap())
}
