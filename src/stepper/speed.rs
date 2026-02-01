pub trait SpeedSource {
    type Error;
    fn speed_rpm(&mut self) -> Result<u32, Self::Error>;
}

pub struct FixedSpeed(pub u32);

impl SpeedSource for FixedSpeed {
    type Error = core::convert::Infallible;
    fn speed_rpm(&mut self) -> Result<u32, Self::Error> {
        Ok(self.0)
    }
}
