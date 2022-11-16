use wpilib::ds::{DriverStation, JoystickPort};

trait Input {
    // Functions we need to implement for a given input mechanism.

    fn boost(&self); //Go fast

    fn turtle(&self); //Go slow
    
    fn intake(&self);
    
    fn lefttankaxis(&self);
    
    fn righttankaxis(&self);
}

struct XboxController {
    controllerPort: JoystickPort,
}

impl Input for XboxController {
    fn boost(&self) {
        todo!()
    }

    fn turtle(&self) {
        todo!()
    }

    fn intake(&self) {
        todo!()
    }

    fn lefttankaxis(&self) {
        todo!()
    }

    fn righttankaxis(&self) {
        todo!()
    }
}
