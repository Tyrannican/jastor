use jastor::{CombatLogParser, error::JastorError};

fn main() -> Result<(), JastorError> {
    let infile = "combat.log";
    let mut p = CombatLogParser::default();
    p.parse(infile)?;
    println!("Parsed {} events from \"{infile}\"", p.total_events());

    Ok(())
}
