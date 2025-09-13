use super::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct MockSps30 {
    start_time: Instant,
    counter: Arc<Mutex<u32>>,
}

impl MockSps30 {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn wake_up(&mut self) -> Result<(), SensorError> {
        println!("[MOCK] Sensor woken up");
        Ok(())
    }

    pub fn start_measurement(&mut self) -> Result<(), SensorError> {
        println!("[MOCK] Started measurements");
        Ok(())
    }

    pub fn read_data_ready_flag(&mut self) -> Result<bool, SensorError> {
        // Simulate data being ready every second
        let elapsed = self.start_time.elapsed().as_secs();
        Ok(elapsed > 0 && elapsed % 1 == 0)
    }

    pub fn read_measured_values(&mut self) -> Result<AirInfo, SensorError> {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        
        // Generate realistic-looking mock data with some variation
        let base_value = (*counter as f32) * 0.1;
        let noise = (base_value * 0.1).sin() * 2.0;
        
        Ok(AirInfo {
            mass_pm1_0: 5.0 + base_value + noise,
            mass_pm2_5: 12.0 + base_value * 1.5 + noise,
            mass_pm4_0: 18.0 + base_value * 2.0 + noise,
            mass_pm10: 25.0 + base_value * 2.5 + noise,
            number_pm0_5: 150.0 + base_value * 10.0 + noise * 5.0,
            number_pm1_0: 89.0 + base_value * 8.0 + noise * 4.0,
            number_pm2_5: 45.0 + base_value * 6.0 + noise * 3.0,
            number_pm4_0: 23.0 + base_value * 4.0 + noise * 2.0,
            number_pm10: 12.0 + base_value * 2.0 + noise,
            typical_size: 0.8 + (base_value * 0.01).sin() * 0.1,
        })
    }

    pub fn stop_measurement(&mut self) -> Result<(), SensorError> {
        println!("[MOCK] Stopped measurements");
        Ok(())
    }

    pub fn sleep(&mut self) -> Result<(), SensorError> {
        println!("[MOCK] Sensor sleeping");
        Ok(())
    }

    pub fn read_device_product_type(&mut self) -> Result<[u8; 32], SensorError> {
        let mut bytes = [0u8; 32];
        let product_type = b"SPS30";
        bytes[..product_type.len()].copy_from_slice(product_type);
        Ok(bytes)
    }

    pub fn read_device_serial_number(&mut self) -> Result<[u8; 32], SensorError> {
        let mut bytes = [0u8; 32];
        let serial = b"MOCK_SERIAL_001";
        bytes[..serial.len()].copy_from_slice(serial);
        Ok(bytes)
    }

    pub fn read_firmware_version(&mut self) -> Result<[u8; 32], SensorError> {
        let mut bytes = [0u8; 32];
        let version = b"2.2";
        bytes[..version.len()].copy_from_slice(version);
        Ok(bytes)
    }

    pub fn start_fan_cleaning(&mut self) -> Result<(), SensorError> {
        println!("[MOCK] Started fan cleaning");
        Ok(())
    }
}

impl Sensor for MockSps30 {
    fn wake_up(&mut self) -> Result<(), SensorError> { 
        MockSps30::wake_up(self) 
    }
    fn start_measurement(&mut self) -> Result<(), SensorError> { 
        MockSps30::start_measurement(self) 
    }
    fn read_data_ready_flag(&mut self) -> Result<bool, SensorError> { 
        MockSps30::read_data_ready_flag(self) 
    }
    fn read_measured_values(&mut self) -> Result<AirInfo, SensorError> { 
        MockSps30::read_measured_values(self) 
    }
    fn stop_measurement(&mut self) -> Result<(), SensorError> { 
        MockSps30::stop_measurement(self) 
    }
    fn sleep(&mut self) -> Result<(), SensorError> { 
        MockSps30::sleep(self) 
    }
    fn start_fan_cleaning(&mut self) -> Result<(), SensorError> { 
        MockSps30::start_fan_cleaning(self) 
    }
    
    // Device information methods
    fn read_device_product_type(&mut self) -> Result<[u8; 32], SensorError> { 
        MockSps30::read_device_product_type(self) 
    }
    fn read_device_serial_number(&mut self) -> Result<[u8; 32], SensorError> { 
        MockSps30::read_device_serial_number(self) 
    }
    fn read_firmware_version(&mut self) -> Result<[u8; 32], SensorError> { 
        MockSps30::read_firmware_version(self) 
    }
}
