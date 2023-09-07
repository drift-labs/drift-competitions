use drift::error::DriftResult;

pub fn get_random_draw(max: u128) -> DriftResult<u128> {
    let random_number = max / 2; // todo: replace with VRF

    Ok(random_number)
}
