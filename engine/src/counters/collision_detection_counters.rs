use crate::counters::Timer;
use std::fmt::{Display, Formatter, Result};

#[derive(Default, Clone, Copy)]
pub struct CollisionDetectionCounters {
    /// Number of contact pairs detected
    pub ncontact_pairs: usize,
    /// Time spent for the broad-phase of the collision detection
    pub broad_phase_time: Timer,
    /// Time spent for the narrow-phase of the collision detection
    pub narrow_phase_time: Timer,
}

impl CollisionDetectionCounters {
    pub fn new() -> Self {
        Self {
            ncontact_pairs: 0,
            broad_phase_time: Timer::new(),
            narrow_phase_time: Timer::new(),
        }
    }
}

impl Display for CollisionDetectionCounters {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Number of contact pairs: {}", self.ncontact_pairs)?;
        writeln!(f, "Broad-phase time: {}", self.broad_phase_time)?;
        writeln!(f, "Narrow-phase time: {}", self.narrow_phase_time)
    }
}
