use jastor::{CombatLogParser, error::JastorError};

fn main() -> Result<(), JastorError> {
    for infile in [
        "logs/combat.log",
        "logs/combat2.log",
        "logs/combat3.log",
        "logs/combat4.log",
        "logs/combat5.log",
    ] {
        let mut p = CombatLogParser::default();
        p.parse(infile)?;
        println!("Parsed {} events from \"{infile}\"", p.total_events());
    }

    Ok(())
}
