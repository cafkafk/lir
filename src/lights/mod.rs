pub mod backlights;

trait Light {
    fn set_brightness(&self);
}
