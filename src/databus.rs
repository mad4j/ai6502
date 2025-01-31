pub trait DataBus {
    fn new() -> Self;
    fn memory_size(&self) -> usize;
    fn reset(&mut self);
    fn set_byte(&mut self, address: u16, value: u8);
    fn get_byte(&self, address: u16) -> u8;
    
}

pub struct SimpleDataBus {
    memory: [u8; 65536],
}

impl SimpleDataBus {
    pub fn new() -> Self {
        SimpleDataBus {
            memory: [0; 65536],
        }
    }
}

impl DataBus for SimpleDataBus {

    fn new() -> Self {
        SimpleDataBus {
            memory: [0; 65536],
        }
    }

    fn memory_size(&self) -> usize {
        self.memory.len()
    }

    fn reset(&mut self) {
        for i in 0..self.memory_size() {
            self.set_byte(i as u16, 0);
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn get_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_byte() {
        let mut bus = SimpleDataBus::new();
        bus.set_byte(0x1234, 0xAB);
        assert_eq!(bus.get_byte(0x1234), 0xAB);
    }

    #[test]
    fn test_default_value() {
        let bus = SimpleDataBus::new();
        assert_eq!(bus.get_byte(0x1234), 0x00);
    }

    #[test]
    fn test_memory_size() {
        let bus = SimpleDataBus::new();
        assert_eq!(bus.memory_size(), 65536);
    }           
}