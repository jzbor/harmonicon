use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

use notify::{RecursiveMode, Watcher};

use crate::driver::HarmoniconDriver;
use crate::error::HarmoniconError;

pub fn start_reload_thread(file: PathBuf) -> Receiver<HarmoniconDriver> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || reload_thread(file, tx));
    rx
}

fn reload_thread(file: PathBuf, tx: Sender<HarmoniconDriver>) {
    // TODO: replace unwraps
    let (event_tx, event_rx) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(event_tx).unwrap();
    watcher.watch(&file, RecursiveMode::NonRecursive).unwrap();

    for res in event_rx {
        if let Err(e) = res {
            HarmoniconError::from(e).warn();
            continue;
        } else if let Ok(e) = res && !e.kind.is_modify() && !e.kind.is_create() {
            continue;
        } else if !fs::exists(&file).unwrap_or(true) {
            continue;
        }

        println!("reloading...");
        match HarmoniconDriver::parse_from_file(&file) {
            Ok(driver) => tx.send(driver).unwrap(),
            Err(e) => e.warn(),
        }
        watcher.watch(&file, RecursiveMode::NonRecursive).unwrap();
    }
}
