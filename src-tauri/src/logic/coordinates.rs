use crate::{
    constants::offsets::{COORD_X_OFFSET, COORD_Y_OFFSET, COORD_Z_OFFSET, WORLD_CHR_MAN_OFFSET},
    memory::Application,
    models::response::Coordinate,
};

pub fn get_my_coordinate(memory: &mut Application) -> Result<Coordinate, String> {
    if !memory.is_running {
        return Err("Target process no longer running".to_string());
    }

    let x = memory
        .read::<f64>(WORLD_CHR_MAN_OFFSET, COORD_X_OFFSET)
        .ok_or("Failed to read X")?;

    let y = memory
        .read::<f64>(WORLD_CHR_MAN_OFFSET, COORD_Y_OFFSET)
        .ok_or("Failed to read Y")?;

    let z = memory
        .read::<f64>(WORLD_CHR_MAN_OFFSET, COORD_Z_OFFSET)
        .ok_or("Failed to read Z")?;

    Ok(Coordinate { x: Some(x), y: Some(y), z: Some(z) })
}
