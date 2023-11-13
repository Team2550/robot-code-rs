use input;
use wpilib::pwm::PWM;

  struct Drivetrain {
    front_left_motor: PWM::new(1),
    front_right_motor: PWM::new(2),
    back_left_motor: PWM::new(3),
    back_right_motor: PWM::new(4),
  
    boost_speed: f32,
  }
  
  impl Drivetrain {
    /**
     * @brief Control set for a mecanum drivebase
     */
    pub fn mecanum_drive(&mut self, inputs: &Input) {
      let speed = inputs.lefty()
      let rotation = inputs.rightx()
      
      if inputs.b() { // boost
        self.boost_speed = 1.0;
      } else { // standard
        self.boost_speed = 0.75;
      }
  
      // Mecanum stuff * boost 
      let front_left_speed = (speed + rotation) * self.boost_speed;
      let front_right_speed = (-speed + rotation) * self.boost_speed;
      let back_left_speed = (-speed + rotation) * self.boost_speed;
      let back_right_speed = (speed + rotation) * self.boost_speed;
  
      // Set motor speed
      self.front_left_motor.set_speed(front_left_speed);
      self.front_right_motor.set_speed(front_right_speed);
      self.back_left_motor.set_speed(back_left_speed);
      self.back_right_motor.set_speed(back_right_speed);
    }

    /**
     * @brief Control set for a tank drivebase
     */
    pub fn tank_drive(&mut self, inputs: &Input) {
      let left = inputs.lefty()
      let right = inputs.righty()
      
      if inputs.b() { // boost
        self.boost_speed = 1.0;
      } else { // standard
        self.boost_speed = 0.75;
      }
  
      // Tank stuff * boost 
      let left_speed = (left) * self.boost_speed;
      let right_speed = (right) * self.boost_speed;
  
      // Set motor speed
      self.front_left_motor.set_speed(left_speed);
      self.back_left_motor.set_speed(left_speed);
      self.front_right_motor.set_speed(right_speed);
      self.back_right_motor.set_speed(right_speed);
    }
  }
  
  