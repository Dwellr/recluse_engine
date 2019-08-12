use crate::counters::Timer;
use std::fmt::{Display, Formatter, Result};

#[derive(Default, Clone, Copy)]
pub struct SolverCounters {
    pub nconstraints: usize,
    pub ncontacts: usize,
    pub velocity_resolution_time: Timer,
    pub assembly_time: Timer,
    pub velocity_update: Timer,
    pub position_resolution_time: Timer,
}

impl SolverCounters {
    pub fn new() -> Self {
        Self {
            nconstraints: 0,
            ncontacts: 0,
            assembly_time: Timer::new(),
            velocity_resolution_time: Timer::new(),
            velocity_update: Timer::new(),
            position_resolution_time: Timer::new(),
        }
    }
}

impl Display for SolverCounters {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Number of contacts: {}", self.ncontacts)?;
        writeln!(f, "Number of constraints: {}", self.nconstraints)?;
        writeln!(f, "Assembly time: {}", self.assembly_time)?;
        writeln!(
            f,
            "Velocity resolution time: {}",
            self.velocity_resolution_time
        )?;
        writeln!(f, "Velocity update time: {}", self.velocity_update_time)?;
        writeln!(
            f,
            "Position resolution time: {}",
            self.position_resolution_time
        )
    }
}
