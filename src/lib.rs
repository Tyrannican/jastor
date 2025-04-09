pub mod error;
pub mod event;

use error::JastorError;
use event::*;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Default)]
pub struct CombatLogParser {
    size: usize,
}

impl CombatLogParser {
    pub fn parse(&mut self, infile: impl AsRef<Path>) -> Result<(), JastorError> {
        let fh =
            File::open(infile.as_ref()).map_err(|e| JastorError::FileReadError(e.to_string()))?;
        let reader = BufReader::new(fh);

        for line in reader.lines() {
            let line = line.map_err(|e| JastorError::ParseError(e.to_string()))?;
            self.parse_event(&line)?;
            self.size += 1;
        }

        Ok(())
    }

    pub fn total_events(&self) -> usize {
        self.size
    }

    fn parse_event(&mut self, line: &str) -> Result<(), JastorError> {
        let Some((ts, event)) = line.split_once("  ") else {
            return Err(JastorError::ParseError(format!(
                "expected timestamp with 2 spaces - got {line}"
            )));
        };

        let Some((event_type, args)) = event.split_once(',') else {
            return Err(JastorError::ParseError(format!(
                "expected event type to be present - got {event}"
            )));
        };

        let event = EventType::from_str(&event_type);
        // Advanced parameter fields:
        // 1. GUID
        // 2. Owner GUID (00000000000000000)
        // 3. Current HP
        // 4. Max HP
        // 5. Attack Power
        // 6. Spell Power
        // 7 ? - Armor apparently but no dice
        // 8. ? - Absorb shield
        // 9. ?
        // 10. ?
        // 11. Power Type
        // 12. Current Power
        // 13. Max Power
        // 14. Power Cost
        // 15. X coord
        // 16. Y Coord
        // 17. Map ID
        // 18. Facing Direction
        // 19. Level (NPC) or iLvl (Player)
        //
        // Only Need GUID -> Max HP and Current Power -> Level

        event.is_valid()?;
        if event.skip() {
            return Ok(());
        }

        Ok(())
    }
}
