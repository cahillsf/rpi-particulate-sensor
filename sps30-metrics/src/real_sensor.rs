use super::*;
use sps30_i2c::Sps30;
use linux_embedded_hal::{Delay, I2cdev};

pub struct RealSps30 {
    sensor: Sps30<I2cdev, Delay>,
}

impl RealSps30 {
    pub fn new() -> Result<Self, SensorError> {
        let dev = I2cdev::new("/dev/i2c-1")
            .map_err(|e| SensorError::Init { 
                message: format!("Failed to open I2C device: {}", e) 
            })?;
        let delay = Delay;
        let sensor = Sps30::new_sps30(dev, delay);
        Ok(Self { sensor })
    }

    pub fn wake_up(&mut self) -> Result<(), SensorError> {
        Ok(self.sensor.wake_up()?)
    }

    pub fn start_measurement(&mut self) -> Result<(), SensorError> {
        Ok(self.sensor.start_measurement()?)
    }

    pub fn read_data_ready_flag(&mut self) -> Result<bool, SensorError> {
        Ok(self.sensor.read_data_ready_flag()?)
    }

    pub fn read_measured_values(&mut self) -> Result<AirInfo, SensorError> {
        Ok(self.sensor.read_measured_values()?)
    }

    pub fn stop_measurement(&mut self) -> Result<(), SensorError> {
        Ok(self.sensor.stop_measurement()?)
    }

    pub fn sleep(&mut self) -> Result<(), SensorError> {
        Ok(self.sensor.sleep()?)
    }

    pub fn read_device_product_type(&mut self) -> Result<[u8; 8], SensorError> {
        Ok(self.sensor.read_device_product_type()?)
    }

    pub fn read_device_serial_number(&mut self) -> Result<[u8; 32], SensorError> {
        Ok(self.sensor.read_device_serial_number()?)
    }

    pub fn read_firmware_version(&mut self) -> Result<[u8; 8], SensorError> {
        Ok(self.sensor.read_firmware_version()?)
    }

    pub fn start_fan_cleaning(&mut self) -> Result<(), SensorError> {
        Ok(self.sensor.start_fan_cleaning()?)
    }
}

impl Sensor for RealSps30 {
    fn wake_up(&mut self) -> Result<(), SensorError> { 
        RealSps30::wake_up(self) 
    }
    fn start_measurement(&mut self) -> Result<(), SensorError> { 
        RealSps30::start_measurement(self) 
    }
    fn read_data_ready_flag(&mut self) -> Result<bool, SensorError> { 
        RealSps30::read_data_ready_flag(self) 
    }
    fn read_measured_values(&mut self) -> Result<AirInfo, SensorError> { 
        RealSps30::read_measured_values(self) 
    }
    fn stop_measurement(&mut self) -> Result<(), SensorError> { 
        RealSps30::stop_measurement(self) 
    }
    fn sleep(&mut self) -> Result<(), SensorError> { 
        RealSps30::sleep(self) 
    }
    fn start_fan_cleaning(&mut self) -> Result<(), SensorError> { 
        RealSps30::start_fan_cleaning(self) 
    }
    
    // Device information methods
    fn read_device_product_type(&mut self) -> Result<[u8; 8], SensorError> { 
        RealSps30::read_device_product_type(self) 
    }
    fn read_device_serial_number(&mut self) -> Result<[u8; 8], SensorError> { 
        RealSps30::read_device_serial_number(self) 
    }
    fn read_firmware_version(&mut self) -> Result<[u8; 8], SensorError> { 
        RealSps30::read_firmware_version(self) 
    }
}
