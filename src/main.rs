use eyre::Context;
use jastor::parser::EventLogParser;
use std::io::BufReader;

fn main() -> eyre::Result<()> {
    let paths = std::fs::read_dir("./logs")?;
    for path in paths {
        let entry = path?;
        let f = std::fs::File::open(entry.path()).wrap_err_with(|| {
            format!("failed to read combat log file {}", entry.path().display())
        })?;

        let reader = BufReader::new(f);
        let parser = EventLogParser::new(reader);

        for event in parser.into_iter() {
            eprintln!("{:?}", event?);
        }
    }

    Ok(())
}
