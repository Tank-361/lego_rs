extern crate ev3dev_lang_rust;
use lib::tank_drive;
use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::GyroSensor;
use std::time::Duration;

fn main() -> Ev3Result<()> {
    let left_motor = LargeMotor::get(MotorPort::OutB)?;
    let right_motor = LargeMotor::get(MotorPort::OutC)?;
    let gyro_sensor = GyroSensor::find()?;

    let drive: tank_drive::DriveTrain = tank_drive::DriveTrain {
        right: right_motor,
        left: left_motor,
        gyro: gyro_sensor,
    };

    drive.move_direction(tank_drive::Direction::Forward, Duration::from_secs(2))?;
    Ok(())
}
