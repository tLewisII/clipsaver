use std::thread::sleep;
use std::time::Duration;
mod pasteboard;
mod tot;
use pasteboard::get_pasteboard;
use structopt::StructOpt;
use tot::prepend_to_tot;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt, Debug)]
#[structopt(name = "clipboard")]
struct Options {
    /// Which tot dot to use for the clipboard history.
    #[structopt(long, default_value = "7")]
    dot: u64,

    /// How often the clipboard should be polled.
    #[structopt(long, default_value = "1")]
    polling_interval: u64,

    /// If you want logging when clipsaver does something, use this.
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() -> Result<()> {
    let args: Options = Options::from_args();
    let mut last_value = get_pasteboard()?;
    loop {
        sleep(Duration::from_secs(args.polling_interval));
        let current_value = get_pasteboard()?;
        if last_value == current_value {
            continue;
        }
        last_value = current_value;
        prepend_to_tot(args.dot, &last_value)?;
        if args.verbose >= 1 {
            let log = format!("Prepended value to dot {}.", args.dot);
            println!("{}", log);
        }
    }
}
