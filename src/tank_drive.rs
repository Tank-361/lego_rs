use ev3dev_lang_rust::motors::LargeMotor;
use ev3dev_lang_rust::sensors::GyroSensor;
use ev3dev_lang_rust::Ev3Result;
use std::time::Duration;

pub struct DriveTrain {
   pub right: LargeMotor,
   pub left: LargeMotor, 
   pub gyro: GyroSensor,
}

pub enum Direction {
    ClockWise, 
    AntiClockWise,
    Forward,
    Backward,
}

impl DriveTrain {
    pub fn get_right(&self) -> &LargeMotor { return &self.right; }
    pub fn get_left(&self) -> &LargeMotor { return &self.left; }
    pub fn get_gyro(&self) -> &GyroSensor { return &self.gyro; }

    pub fn move_direction(&self, direction: Direction, time: Duration) -> Ev3Result<()> {
        let duration = Some(time);
        match direction {
            Direction::ClockWise => {
                self.right.run_timed(duration)?;
                self.right.set_duty_cycle_sp(50)?;
                self.left.run_timed(duration)?;
                self.left.set_duty_cycle_sp(-50)?;
                Ok(())
            }
            Direction::AntiClockWise => {
                self.right.run_timed(duration)?;
                self.right.set_duty_cycle_sp(-50)?;
                self.left.run_timed(duration)?;
                self.left.set_duty_cycle_sp(50)?;
                Ok(())
            }
            Direction::Forward => {
                self.right.run_timed(duration)?;
                self.right.set_duty_cycle_sp(50)?;
                self.left.run_timed(duration)?;
                self.left.set_duty_cycle_sp(50)?;
                Ok(())
            }
            Direction::Backward => {
                self.right.run_timed(duration)?;
                self.right.set_duty_cycle_sp(-50)?;
                self.left.run_timed(duration)?;
                self.left.set_duty_cycle_sp(-50)?;
                Ok(())
            }
            }
        }
    }  
