use wpilib::ds::{*};

trait Input {
  // Functions we need to implement for a given input mechanism.

  fn boost(&self) -> bool; //Go fast

  fn turtle(&self) -> bool; //Go slow
  
  fn intake(&self) -> bool;

  fn extend(&self) -> bool; //Extend pison

  fn retract(&self) -> bool; //Retract piston
  
  fn lefttankaxis(&self) -> f32;
  
  fn righttankaxis(&self) -> f32;

  const A: u8 = 1;
  const B: u8 = 2;
	const X: u8 = 3;
	const Y: u8 = 4;

  const LB: u8 = 5;
  const RB: u8 = 6;

  const BACK: u8 = 7;
  const START: u8 = 8;

}

struct XboxController<'a> {
  ds: &'a DriverStation<'a>,
  port: JoystickPort,
}

impl XboxController<'_> {
  fn y(&self) -> f32 {
      self.ds
          .stick_axis(self.port, JoystickAxis::new(1).unwrap())
          .unwrap_or(0.0)
  }

  fn raw_button(&self, button: u8) -> bool {
      self.ds.stick_button(self.port, button).unwrap_or(false)
  }
}


impl Input for XboxController<'_> {

fn boost(&self) -> bool {
    self.raw_button(Self::B)
  }

  fn turtle(&self) -> bool {
    self.raw_button(Self::A)
  }

  fn intake(&self) -> bool {
    self.raw_button(Self::X)
  }

  fn extend(&self) -> bool {
    self.raw_button(Self::LB)
  }

  fn retract(&self) -> bool {
    self.raw_button(Self::RB)
  }

  fn lefttankaxis(&self) -> f32 {
    self.y() //Figure out how to specify which stick
  }

  fn righttankaxis(&self) -> f32 {
    self.y() //Figure out how to specify which stick
  }
}
