use rppal::gpio::{Gpio, OutputPin};
use tm1637_embedded_hal::blocking::SevenSegment;
use crate::DisplayError;

pub struct RealDisplay {
    tm1637: tm1637_embedded_hal::TM1637Blocking<OutputPin, OutputPin>,
}

impl RealDisplay {
    pub fn new(clk_pin: u8, dio_pin: u8) -> Result<Self, DisplayError> {
        let gpio = Gpio::new().map_err(|e| DisplayError::Init {
            message: format!("Failed to initialize GPIO: {}", e),
        })?;

        let clk = gpio
            .get(clk_pin)
            .map_err(|e| DisplayError::Init {
                message: format!("Failed to get CLK pin {}: {}", clk_pin, e),
            })?
            .into_output();

        let dio = gpio
            .get(dio_pin)
            .map_err(|e| DisplayError::Init {
                message: format!("Failed to get DIO pin {}: {}", dio_pin, e),
            })?
            .into_output();

        let mut tm1637 = tm1637_embedded_hal::TM1637Blocking::new(clk, dio);

        tm1637.init().map_err(|e| DisplayError::Communication {
            message: format!("Failed to initialize TM1637: {:?}", e),
        })?;

        println!("📺 Real TM1637 Display: Initialized on pins CLK={}, DIO={}", clk_pin, dio_pin);
        
        Ok(RealDisplay { tm1637 })
    }
}

impl super::Display for RealDisplay {
    fn show_value(&mut self, value: f32) -> Result<(), DisplayError> {
        // Round to 1 decimal place for display
        let rounded = (value * 10.0).round() / 10.0;
        
        // Convert to integer representation (e.g., 12.3 becomes 123)
        let int_value = (rounded * 10.0) as u16;
        
        // Split into digits
        let digit0 = (int_value / 100) % 10;
        let digit1 = (int_value / 10) % 10;
        let digit2 = int_value % 10;
        
        // Create segments array with decimal point on second digit
        let segments = [
            tm1637_embedded_hal::mappings::SegmentBits::encode_digit(digit0 as u8),
            tm1637_embedded_hal::mappings::SegmentBits::encode_digit(digit1 as u8) | 0b1000_0000, // Add decimal point
            tm1637_embedded_hal::mappings::SegmentBits::encode_digit(digit2 as u8),
            0x00, // Blank the 4th digit
        ];
        
        self.tm1637
            .write_segments_raw(0, &segments)
            .map_err(|e| DisplayError::Communication {
                message: format!("Failed to display value: {:?}", e),
            })?;

        println!("📺 Real Display: PM2.5 = {:.1} µg/m³", rounded);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), DisplayError> {
        self.tm1637.clear().map_err(|e| DisplayError::Communication {
            message: format!("Failed to clear display: {:?}", e),
        })?;
        
        println!("📺 Real Display: Cleared");
        Ok(())
    }
}

