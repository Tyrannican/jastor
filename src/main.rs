use eyre::Context;
use jastor::{event::Event, parser::EventLogParser};
use std::io::BufReader;

fn main() -> eyre::Result<()> {
    let paths = std::fs::read_dir("./logs")?;
    for path in paths {
        let entry = path?;
        eprintln!("Parsing {}", entry.path().display());
        let f = std::fs::File::open(entry.path()).wrap_err_with(|| {
            format!("failed to read combat log file {}", entry.path().display())
        })?;

        let reader = BufReader::new(f);
        let parser = EventLogParser::new(reader);

        for event in parser.into_iter() {
            let event = event?;
            match event.event {
                Event::LogVersion(log_version) => {
                    if log_version.build.as_str() < "12.0.0" {
                        eprintln!("Only supporting Midnight and beyond!");
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
