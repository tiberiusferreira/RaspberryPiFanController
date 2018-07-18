use sysfs_gpio::{Pin, Direction};
const FAN_PIN_NUMBER: u64 = 9;


pub struct Fan{
    pin: Pin
}

impl Fan{
    pub fn new() -> Self{
        let fan_pin = Pin::new(FAN_PIN_NUMBER);
        fan_pin.export().expect(&format!("Could not export pin {} to user space.", FAN_PIN_NUMBER));
        fan_pin.set_direction(Direction::Out).expect("Could not set pin {} direction to Out");
        Fan{
            pin: fan_pin
        }
    }
    pub fn turn_on_fan(&self){
        self.pin.set_value(1).expect(&format!("Could not set pin {} to 1", FAN_PIN_NUMBER));
    }
    pub fn turn_off_fan(&self){
        self.pin.set_value(0).expect(&format!("Could not set pin {} to 0", FAN_PIN_NUMBER));
    }
}
