use jastor::LogFile;

fn main() -> eyre::Result<()> {
    let paths = std::fs::read_dir("./logs")?;

    for path in paths {
        let entry = path?;
        eprintln!("Parsing {}", entry.path().display());

        // Safety: We can guarantee that the file is not modified underneath
        // as these are static logs that don't change
        LogFile::parse(entry.path())?;
    }

    Ok(())
}
