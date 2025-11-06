use crate::DisplayError;

pub struct MockDisplay {
    last_value: Option<f32>,
}

impl MockDisplay {
    pub fn new() -> Self {
        println!("📺 Mock Display: Initialized");
        MockDisplay { last_value: None }
    }
}

impl super::Display for MockDisplay {
    fn show_value(&mut self, value: f32) -> Result<(), DisplayError> {
        self.last_value = Some(value);
        println!("📺 Mock Display: PM2.5 = {:.1} µg/m³", value);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), DisplayError> {
        self.last_value = None;
        println!("📺 Mock Display: Cleared");
        Ok(())
    }
}

