pub mod event;
pub mod parser;
pub mod player;
pub mod types;

use memmap::MmapOptions;
use parser::{EventLogParser, ParsedEvent};
use std::{io::BufReader, path::Path};
pub struct LogFile;

impl LogFile {
    pub fn parse(path: impl AsRef<Path>) -> eyre::Result<Vec<ParsedEvent>> {
        let file = std::fs::File::open(path)?;

        // Safety: We can guarantee that the file is not modified underneath
        // as these are static logs that don't change
        let map = unsafe { MmapOptions::new().map(&file)? };
        std::thread::scope(|scope| {
            let n = std::thread::available_parallelism().expect("a valid value for parallelism");
            let chunk = map.len() / n.get();
            let mut at = 0;
            let (tx, rx) = std::sync::mpsc::sync_channel::<Vec<ParsedEvent>>(n.get());

            for _ in 0..n.get() {
                let start = at;
                let end = (at + chunk).min(map.len());
                let end = if end == map.len() {
                    map.len()
                } else {
                    let buf = &map[end..];
                    let newline = buf.iter().position(|b| *b == b'\n').unwrap_or(buf.len());
                    end + newline + 1
                };
                let reader = BufReader::new(&map[start..end]);
                let parser = EventLogParser::new(reader);
                at = end;

                let tx = tx.clone();
                scope.spawn(move || {
                    tx.send(
                        parser
                            .into_iter()
                            .map(|event| event.expect("a valid event"))
                            .collect::<Vec<ParsedEvent>>(),
                    )
                });
            }

            drop(tx);
            let mut events = rx.into_iter().collect::<Vec<Vec<ParsedEvent>>>();
            events.sort_by(|a, b| a[0].timestamp.cmp(&b[0].timestamp));
            Ok(events.into_iter().flatten().collect::<Vec<ParsedEvent>>())
        })
    }
}

#[cfg(test)]
mod jastor_tests {
    use super::*;

    #[test]
    fn it_parses_all_the_files_without_error() -> eyre::Result<()> {
        let paths = std::fs::read_dir("./logs")?;

        for path in paths {
            let entry = path?;
            eprintln!("Parsing {}", entry.path().display());
            LogFile::parse(entry.path())?;
        }

        Ok(())
    }
}
