# Jastor - World of Warcraft Combat Log Parser

Project to parse WoW combat logs.

Just playing around for funsies

# Notes

Some notes on what I've found as this API is poorly documented and most of it is just heresay:

## Events

Information related to events

### Base Events

There are 8 fields in the base parameters:

* `src_guid`: The GUID for the Source unit
* `src_name`: Name of the Source unit (i.e. "Player-Realm-Region" or "Rik Reverb")
* `src_flags`: The flags associated with the unit (Player/NPC, Hostile/Friendly, etc)
* `src_raid_flags`: Raid markers on the unit
* `target_guid`: The GUID for the Target unit
* `target_name`: Name of the Target unit (i.e. "Player-Realm-Region" or "Rik Reverb")
* `target_flags`: The flags associated with the unit (Player/NPC, Hostile/Friendly, etc)
* `target_raid_flags`: Raid markers on the unit
