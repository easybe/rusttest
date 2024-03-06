use log::{info, debug};
use std::thread;
use std::time::SystemTime;

const LOG_FILE: &str = "out.log";

fn init_log() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file(LOG_FILE).unwrap())
        .apply()
        .unwrap();
}

fn main() {
    init_log();

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        for i in 1..=3 {
            info!("Start");
            debug!("{i}");
            info!("End");
            std::fs::copy(LOG_FILE, format!("{i}.log")).unwrap();
            std::fs::File::create(LOG_FILE).unwrap();
        }

        tx.send("Done.").unwrap(); // Send message to main thread
    });

    for msg in rx {
        println!("{}", msg);
    }
}
