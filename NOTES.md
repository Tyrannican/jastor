# Notes

Some notes on what I've found as this API is poorly documented and most of it is just heresay.

This doc is related to the CLEU (Combat Log Event Unfiltered) events.

Some of this information is taken from the [(outdated) wiki page](https://warcraft.wiki.gg/wiki/COMBAT_LOG_EVENT) on Combat Events

## Events

Information related to events

### Base Parameters

Some events (mostly damage etc.) have what is called `Base Parameters`.
This is essentially just information relating to the `Source` unit (i.e. the one performing the action) and the `Target` unit (the one being targetted by the event).

Each "unit" has a series of flags:

* [Unit Flags](https://warcraft.wiki.gg/wiki/UnitFlag)
    * This determines if the unit is a Player / NPC, Hostile / Friendly etc.
* [Raid Flags](https://warcraft.wiki.gg/wiki/RaidFlag)
    * Any Raid markers attached to the unit (Square, Circle etc.)

There are 8 fields in the base parameters:

* `src_guid`: The GUID for the Source unit
* `src_name`: Name of the Source unit (i.e. "Player-Realm-Region" or "Rik Reverb")
* `src_flags`: The flags associated with the unit (Player/NPC, Hostile/Friendly, etc)
* `src_raid_flags`: Raid markers on the unit
* `target_guid`: The GUID for the Target unit
* `target_name`: Name of the Target unit (i.e. "Player-Realm-Region" or "Rik Reverb") 
* `target_flags`: The flags associated with the unit (Player/NPC, Hostile/Friendly, etc)
* `target_raid_flags`: Raid markers on the unit

### Prefix Parameters

Some events (`SPELL_` & `RANGE_`) have 3 parameters (`Prefix Parameters`) which are the Spell information

* `spell_id`: The ID of the Spell (internal WoW ID)
* `spell_name`: String containing the name of the spell (e.g. `Arcane Intellect`)
* `spell_school`: The "school" of the spell (e.g. Arcane, Frostfire, etc.)
    * This is in the form of a flag that has to be parsed

### Damage Event

Damage events have the following parameters:

* `amount`: Amount of damage dealt
* `base_amount`: Amount of damage dealt before modifiers (crits etc.)
* `overkill`: Amount of overkill damage (-1 if no overkill)
* `school`: School of damage (Physical, Arcane, etc.)
* `resisted`: How much was resisted
* `blocked`: How much was blocked
* `absorbed`: How much was absorbed
* `critical`: Boolean if the damage was a crit
* `glancing`: Boolean if the damage was a glancing blow
* `is_offhand`: Boolean if the damage was caused by an offhand
* `damage_type`: Spells / Range damage have this parameter
    * `ST`: For single target Damage
    * `AOE`: For AoE damage
    * `SWING_DAMAGE` and `SWING_DAMAGE_LANDED` **DO NOT** have this parameter
