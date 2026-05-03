use jastor::LogFile;

// TODO: Run so that we can gather encounters
fn main() -> eyre::Result<()> {
    for event in LogFile::parse("./logs/WoWCombatLog-041926_195840.txt")? {
        //
    }

    Ok(())
}
