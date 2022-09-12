use std::error::Error;

#[derive(Debug, Clone)]
pub enum SettingValue {
    Float(f32),
    Integer(usize),
}

impl SettingValue {
    pub fn to_string(&self) -> String {
        match self {
            SettingValue::Float(s) => s.to_string(),
            SettingValue::Integer(s) => s.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Setting {
    pub value: SettingValue,
    pub description: String,
}

impl Setting {
    pub fn new(value: SettingValue, description: &str) -> Self {
        Setting {
            value,
            description: description.into(),
        }
    }

    pub fn try_update_value(&mut self, value: &str) -> Result<(), Box<dyn Error>> {
        match self.value {
            SettingValue::Float(_) => self.value = SettingValue::Float(value.parse::<f32>()?),
            SettingValue::Integer(_) => self.value = SettingValue::Integer(value.parse::<usize>()?),
        }

        Ok(())
    }
}
