use crate::{
    constants::offsets::{
        DEATH_TOTAL_OFFSET, GAME_DATA_MAN_OFFSET, GAME_MAN_OFFSET, GENDER_OFFSET, GREAT_RUNE_OFFSET,
        LAST_GRACE_OFFSET, LEVEL_OFFSET, NICKNAME_OFFSET, PLAYTIME_OFFSET, RUNES_HELD_OFFSET,
        RUNE_ARC_OFFSET, TOTAL_RUNES_OBTAINED_OFFSET,
    },
    logic::{
        attributes::{
            get_my_arcane, get_my_dexterity, get_my_endurance, get_my_faith, get_my_hp, get_my_intelligence,
            get_my_max_hp, get_my_max_mp, get_my_max_sp, get_my_mind, get_my_mp, get_my_sp, get_my_strength,
            get_my_vigor,
        },
        coordinates::get_my_coordinate,
        resistances::{
            get_my_focus_madness, get_my_focus_sleep, get_my_immunity_poison, get_my_immunity_scarlet_rot,
            get_my_max_focus_madness, get_my_max_focus_sleep, get_my_max_immunity_poison,
            get_my_max_immunity_scarlet_rot, get_my_max_robustness_frostbite,
            get_my_max_robustness_hemorrhage, get_my_max_vitality_deathblight, get_my_robustness_frostbite,
            get_my_robustness_hemorrhage, get_my_vitality_deathblight,
        },
    },
    memory::{Application, StringEncoding},
    models::response::{Attribute, BaseAttribute, GreatRune, Player, Resistance, Statistics},
    utils::{get_gender_from_hex, get_great_rune_from_hex, get_site_grace_from_hex},
};

fn get_my_nickname(memory: &mut Application) -> Result<String, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let nickname = memory
        .read_string(GAME_DATA_MAN_OFFSET, NICKNAME_OFFSET, 32, StringEncoding::Utf16)
        .ok_or("Failed to read nickname")?;

    Ok(nickname)
}

fn get_my_level(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let level = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, LEVEL_OFFSET)
        .ok_or("Failed to read level")?;

    Ok(level)
}

fn get_my_gender(memory: &mut Application) -> Result<String, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let gender = memory
        .read::<u8>(GAME_DATA_MAN_OFFSET, GENDER_OFFSET)
        .ok_or("Failed to read gender")?;

    Ok(get_gender_from_hex(Some(gender)))
}

fn get_my_runes_held(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let runes_held = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, RUNES_HELD_OFFSET)
        .ok_or("Failed to read runes_held")?;

    Ok(runes_held)
}

fn get_my_total_runes_obtained(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let total_runes_obtained = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, TOTAL_RUNES_OBTAINED_OFFSET)
        .ok_or("Failed to read total runes obtained")?;

    Ok(total_runes_obtained)
}

fn get_my_great_runes(memory: &mut Application) -> Result<GreatRune, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let grunes_u32 = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, GREAT_RUNE_OFFSET)
        .ok_or("Failed to read great runes")?;

    let great_runes = get_great_rune_from_hex(Some(grunes_u32));

    let activated = memory
        .read::<u8>(GAME_DATA_MAN_OFFSET, RUNE_ARC_OFFSET)
        .ok_or("Failed to read rune arc")?;

    Ok(GreatRune {
        name: Some(great_runes),
        activated: Some(activated != 0),
    })
}

fn get_my_last_visited_grace(memory: &mut Application) -> Result<String, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let visited_grace = memory
        .read::<u64>(GAME_MAN_OFFSET, LAST_GRACE_OFFSET)
        .ok_or("Failed to read last visited grace")?;

    Ok(get_site_grace_from_hex(Some(visited_grace)))
}

fn get_my_death_total(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let death_total = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, DEATH_TOTAL_OFFSET)
        .ok_or("Failed to read death total")?;

    Ok(death_total)
}

fn get_my_playtime(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let playtime = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, PLAYTIME_OFFSET)
        .ok_or("Failed to read playtime")?;

    Ok(playtime)
}

pub fn get_player_info(memory: &mut Application) -> Result<Player, String> {
    let nickname = get_my_nickname(memory)?;
    let level = get_my_level(memory)?;
    let runes_held = get_my_runes_held(memory)?;
    let total_runes_obtained = get_my_total_runes_obtained(memory)?;
    let gender = get_my_gender(memory)?;
    let great_runes = get_my_great_runes(memory)?;
    let visited_grace = get_my_last_visited_grace(memory)?;

    let death_total = get_my_death_total(memory)?;
    let playtime = get_my_playtime(memory)?;

    let hp = get_my_hp(memory)?;
    let max_hp = get_my_max_hp(memory)?;
    let fp = get_my_mp(memory)?;
    let max_fp = get_my_max_mp(memory)?;
    let stamina = get_my_sp(memory)?;
    let max_stamina = get_my_max_sp(memory)?;

    let vigor = get_my_vigor(memory)?;
    let mind = get_my_mind(memory)?;
    let endurance = get_my_endurance(memory)?;
    let strength = get_my_strength(memory)?;
    let dexterity = get_my_dexterity(memory)?;
    let intelligence = get_my_intelligence(memory)?;
    let faith = get_my_faith(memory)?;
    let arcane = get_my_arcane(memory)?;

    let immunity_poison = get_my_immunity_poison(memory)?;
    let max_immunity_poison = get_my_max_immunity_poison(memory)?;
    let immunity_scarlet_rot = get_my_immunity_scarlet_rot(memory)?;
    let max_immunity_scarlet_rot = get_my_max_immunity_scarlet_rot(memory)?;
    let robustness_hemorrhage = get_my_robustness_hemorrhage(memory)?;
    let max_robustness_hemorrhage = get_my_max_robustness_hemorrhage(memory)?;
    let vitality_deathblight = get_my_vitality_deathblight(memory)?;
    let max_vitality_deathblight = get_my_max_vitality_deathblight(memory)?;
    let robustness_frostbite = get_my_robustness_frostbite(memory)?;
    let max_robustness_frostbite = get_my_max_robustness_frostbite(memory)?;
    let focus_sleep = get_my_focus_sleep(memory)?;
    let max_focus_sleep = get_my_max_focus_sleep(memory)?;
    let focus_madness = get_my_focus_madness(memory)?;
    let max_focus_madness = get_my_max_focus_madness(memory)?;

    let coords = get_my_coordinate(memory)?;

    let base_attrs = BaseAttribute {
        hp: Some(hp),
        max_hp: Some(max_hp),
        fp: Some(fp),
        max_fp: Some(max_fp),
        stamina: Some(stamina),
        max_stamina: Some(max_stamina),
    };

    let attrs = Attribute {
        vigor: Some(vigor),
        mind: Some(mind),
        endurance: Some(endurance),
        strength: Some(strength),
        dexterity: Some(dexterity),
        intelligence: Some(intelligence),
        faith: Some(faith),
        arcane: Some(arcane),
    };

    let resistances = Resistance {
        immunity_poison: Some(immunity_poison),
        max_immunity_poison: Some(max_immunity_poison),
        immunity_scarlet_rot: Some(immunity_scarlet_rot),
        max_immunity_scarlet_rot: Some(max_immunity_scarlet_rot),
        robustness_hemorrhage: Some(robustness_hemorrhage),
        max_robustness_hemorrhage: Some(max_robustness_hemorrhage),
        vitality_deathblight: Some(vitality_deathblight),
        max_vitality_deathblight: Some(max_vitality_deathblight),
        robustness_frostbite: Some(robustness_frostbite),
        max_robustness_frostbite: Some(max_robustness_frostbite),
        focus_sleep: Some(focus_sleep),
        max_focus_sleep: Some(max_focus_sleep),
        focus_madness: Some(focus_madness),
        max_focus_madness: Some(max_focus_madness),
    };

    let statistics = Statistics {
        total_deaths: Some(death_total),
        total_playtime: Some(playtime),
    };

    Ok(Player {
        nickname: Some(nickname),
        level: Some(level),
        gender: Some(gender),
        class: None,
        runes: Some(runes_held),
        total_runes_obtained: Some(total_runes_obtained),
        last_grace: Some(visited_grace),
        great_runes: Some(great_runes),
        base_attributes: Some(base_attrs),
        attributes: Some(attrs),
        resistances: Some(resistances),
        coordinates: Some(coords),
        statistics: Some(statistics),
    })
}
