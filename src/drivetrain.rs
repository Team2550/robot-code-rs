use input;
use wpilib::{MecanumDrive, Joystick};

  struct Drivetrain {
    front_left_motor: Motor,
    front_right_motor: Motor,
    back_left_motor: Motor,
    back_right_motor: Motor,
  
    boost_speed: f32,
  }
  
  impl Drivetrain {
    pub fn mecanum_drive(&mut self, inputs: &Input) {
      let x_axis = inputs.lefttankaxis();
      let y_axis = inputs.righttankaxis();
  
      // Check if boost
      if inputs.boost() {
        self.boost_speed += 1.0;
      } else {
        self.boost_speed = 0.0;
      }
  
      // Mecanum stuff * boost 
      let front_left_speed = x_axis + y_axis + self.boost_speed;
      let front_right_speed = -x_axis + y_axis + self.boost_speed;
      let back_left_speed = -x_axis + y_axis + self.boost_speed;
      let back_right_speed = x_axis + y_axis + self.boost_speed;
  
      // Motots go fast
      self.front_left_motor.set_speed(front_left_speed);
      self.front_right_motor.set_speed(front_right_speed);
      self.back_left_motor.set_speed(back_left_speed);
      self.back_right_motor.set_speed(back_right_speed);
    }
  }
  
  