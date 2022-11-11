use wpilib::*;

fn main() {
    let robot = RobotBase::new().expect("HAL FAILED");

    // Do some setup

    RobotBase::start_competition();

    // In-match code
}
