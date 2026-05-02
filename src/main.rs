use jastor::{
    LogFile,
    event::{Difficulty, Event, EventType},
    parser::ParsedEvent,
};

#[derive(Debug, Default)]
struct Encounter {
    id: u32,
    name: String,
    difficulty: Difficulty,
    group_size: u32,
    instance_id: u32,
    success: bool,
    fight_time: u32,
    events: Vec<ParsedEvent>,
}

#[derive(Debug)]
struct Trash(pub Vec<ParsedEvent>);

// TODO: Run so that we can gather encounters
fn main() -> eyre::Result<()> {
    let mut in_encounter = false;
    let mut encounters: Vec<Encounter> = Vec::new();
    let mut trash: Vec<ParsedEvent> = Vec::new();

    let mut encounter = Encounter::default();
    for event in LogFile::parse("./logs/WoWCombatLog-041926_195840.txt")? {
        match event.event {
            Event::EncounterStart(enc) => {
                encounter.id = enc.encounter_id;
                encounter.name = enc.encounter_name.to_owned();
                encounter.difficulty = enc.difficulty;
                encounter.group_size = enc.group_size;
                encounter.instance_id = enc.instance_id;

                in_encounter = true;
            }
            Event::EncounterEnd(enc) => {
                encounter.success = enc.success;
                encounter.fight_time = enc.fight_time;
                in_encounter = false;
                encounters.push(encounter);
                encounter = Encounter::default();
            }
            _ => {
                if in_encounter {
                    encounter.events.push(event);
                } else {
                    trash.push(event);
                }
            }
        }
    }

    for enc in encounters {
        eprintln!("Encounter: {}", enc.name);
    }

    Ok(())
}
