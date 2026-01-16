use crate::{
    constants::offsets::{
        FOCUS_MADNESS_OFFSET, FOCUS_SLEEP_OFFSET, IMMUNITY_POISON_OFFSET, IMMUNITY_SCARLET_ROT_OFFSET,
        MAX_FOCUS_MADNESS_OFFSET, MAX_FOCUS_SLEEP_OFFSET, MAX_IMMUNITY_POISON_OFFSET,
        MAX_IMMUNITY_SCARLET_ROT_OFFSET, MAX_ROBUSTNESS_FROSTBITE_OFFSET, MAX_ROBUSTNESS_HEMORRHAGE_OFFSET,
        MAX_VITALITY_DEATHBLIGHT_OFFSET, ROBUSTNESS_FROSTBITE_OFFSET, ROBUSTNESS_HEMORRHAGE_OFFSET,
        VITALITY_DEATHBLIGHT_OFFSET, WORLD_CHR_MAN_OFFSET,
    },
    memory::Application,
};

pub fn get_my_immunity_poison(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let immunity_poison = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, IMMUNITY_POISON_OFFSET)
        .ok_or("Failed to read immunity poison")?;

    Ok(immunity_poison)
}
pub fn get_my_max_immunity_poison(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_immunity_poison = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_IMMUNITY_POISON_OFFSET)
        .ok_or("Failed to read max immunity poison")?;

    Ok(max_immunity_poison)
}
pub fn get_my_immunity_scarlet_rot(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let immunity_scarlet_rot = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, IMMUNITY_SCARLET_ROT_OFFSET)
        .ok_or("Failed to read immunity scarlet rot")?;

    Ok(immunity_scarlet_rot)
}
pub fn get_my_max_immunity_scarlet_rot(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_immunity_scarlet_rot = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_IMMUNITY_SCARLET_ROT_OFFSET)
        .ok_or("Failed to read max immunity scarlet rot")?;

    Ok(max_immunity_scarlet_rot)
}
pub fn get_my_robustness_hemorrhage(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let robustness_hemorrhage = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, ROBUSTNESS_HEMORRHAGE_OFFSET)
        .ok_or("Failed to read robustness hemorrhage")?;

    Ok(robustness_hemorrhage)
}
pub fn get_my_max_robustness_hemorrhage(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_robustness_hemorrhage = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_ROBUSTNESS_HEMORRHAGE_OFFSET)
        .ok_or("Failed to read max robustness hemorrhage")?;

    Ok(max_robustness_hemorrhage)
}
pub fn get_my_vitality_deathblight(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let vitality_deathblight = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, VITALITY_DEATHBLIGHT_OFFSET)
        .ok_or("Failed to read vitality deathblight")?;

    Ok(vitality_deathblight)
}
pub fn get_my_max_vitality_deathblight(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_vitality_deathblight = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_VITALITY_DEATHBLIGHT_OFFSET)
        .ok_or("Failed to read max vitality deathblight")?;

    Ok(max_vitality_deathblight)
}
pub fn get_my_robustness_frostbite(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let robustness_frostbite = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, ROBUSTNESS_FROSTBITE_OFFSET)
        .ok_or("Failed to read robustness frostbite")?;

    Ok(robustness_frostbite)
}
pub fn get_my_max_robustness_frostbite(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_robustness_frostbite = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_ROBUSTNESS_FROSTBITE_OFFSET)
        .ok_or("Failed to read max robustness frostbite")?;

    Ok(max_robustness_frostbite)
}
pub fn get_my_focus_sleep(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let focus_sleep = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, FOCUS_SLEEP_OFFSET)
        .ok_or("Failed to read focus sleep")?;

    Ok(focus_sleep)
}
pub fn get_my_max_focus_sleep(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let max_focus_sleep = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_FOCUS_SLEEP_OFFSET)
        .ok_or("Failed to read max focus sleep")?;

    Ok(max_focus_sleep)
}
pub fn get_my_focus_madness(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let focus_madness = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, FOCUS_MADNESS_OFFSET)
        .ok_or("Failed to read focus madness")?;

    Ok(focus_madness)
}
pub fn get_my_max_focus_madness(memory: &mut Application) -> Result<u32, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }
    let max_focus_madness = memory
        .read::<u32>(WORLD_CHR_MAN_OFFSET, MAX_FOCUS_MADNESS_OFFSET)
        .ok_or("Failed to read max focus madness")?;

    Ok(max_focus_madness)
}
