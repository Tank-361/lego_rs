use ev3dev_lang_rust::motors::LargeMotor;
use ev3dev_lang_rust::Ev3Result;
use std::time::Duration;

pub struct Elevator {
    pub lift: LargeMotor,
}

pub enum Direction {
    Up,
    Down,
}

impl Elevator {
    pub fn get_lift(&self) -> &LargeMotor { return &self.lift; }

    pub fn lift(&self, direction: Direction, time: Duration) -> Ev3Result<()> {
        let duration = Some(time);
        match direction {
            Direction::Up => {
                self.lift.run_timed(duration)?;
                self.lift.set_duty_cycle_sp(50)?;
                Ok(())
            }
            Direction::Down => {
                self.lift.run_timed(duration)?;
                self.lift.set_duty_cycle_sp(-50)?;
                Ok(())
            }
        }
    }
}