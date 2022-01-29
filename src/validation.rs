#[allow(unused_imports)]
#[allow(unused_variables)]
use super::grid::{Coordinate, Grid};

// VALIDATION ONLY

impl Grid {
    pub fn validate_move(
        &self,
        start_coord: &Coordinate,
        end_coord: &Coordinate,
    ) -> Result<(), ()> {
        Ok(())
    }
}
