use crate::{
    constants::offsets::{
        ARCANE_OFFSET, DEXTERITY_OFFSET, ENDURANCE_OFFSET, FAITH_OFFSET, GAME_DATA_MAN_OFFSET, HP_OFFSET, INTELLIGENCE_OFFSET, MAX_HP_OFFSET, MAX_MP_OFFSET, MAX_SP_OFFSET, MIND_OFFSET, MP_OFFSET, SP_OFFSET, STRENGTH_OFFSET, VIGOR_OFFSET, WORLD_CHR_MAN_OFFSET
    },
    memory::Application,
};

pub fn get_my_hp(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let hp = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, HP_OFFSET)
        .ok_or("Failed to read HP")?;

    Ok(hp)
}

pub fn get_my_max_hp(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_hp = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_HP_OFFSET)
        .ok_or("Failed to read Max HP")?;

    Ok(max_hp)
}

pub fn get_my_mp(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let mp = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MP_OFFSET)
        .ok_or("Failed to read MP")?;

    Ok(mp)
}

pub fn get_my_max_mp(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_mp = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_MP_OFFSET)
        .ok_or("Failed to read Max MP")?;

    Ok(max_mp)
}

pub fn get_my_sp(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let sp = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, SP_OFFSET)
        .ok_or("Failed to read SP")?;

    Ok(sp)
}

pub fn get_my_max_sp(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_sp = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_SP_OFFSET)
        .ok_or("Failed to read Max SP")?;

    Ok(max_sp)
}

pub fn get_my_vigor(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let vigor = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, VIGOR_OFFSET)
        .ok_or("Failed to read Vigor")?;

    Ok(vigor)
}
pub fn get_my_mind(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let mind = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, MIND_OFFSET)
        .ok_or("Failed to read Mind")?;

    Ok(mind)
}
pub fn get_my_endurance(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let endurance = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, ENDURANCE_OFFSET)
        .ok_or("Failed to read Endurance")?;

    Ok(endurance)
}
pub fn get_my_strength(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let strength = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, STRENGTH_OFFSET)
        .ok_or("Failed to read Strength")?;

    Ok(strength)
}
pub fn get_my_dexterity(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let dexterity = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, DEXTERITY_OFFSET)
        .ok_or("Failed to read Dexterity")?;

    Ok(dexterity)
}
pub fn get_my_intelligence(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let intelligence = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, INTELLIGENCE_OFFSET)
        .ok_or("Failed to read Intelligence")?;

    Ok(intelligence)
}
pub fn get_my_faith(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let faith = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, FAITH_OFFSET)
        .ok_or("Failed to read Faith")?;

    Ok(faith)
}
pub fn get_my_arcane(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let arcane = memory
        .read::<u32>(GAME_DATA_MAN_OFFSET, ARCANE_OFFSET)
        .ok_or("Failed to read Arcane")?;

    Ok(arcane)
}
