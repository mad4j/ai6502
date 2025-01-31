pub struct Registers {
    pub a: u8,  // Accumulator
    pub x: u8,  // X Register
    pub y: u8,  // Y Register
    pub sp: u8, // Stack Pointer
    pub pc: u16, // Program Counter
    pub status: u8, // Status Register
}

// Status register flags
pub const CARRY_FLAG: u8 = 0b0000_0001;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
pub const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
pub const BREAK_COMMAND_FLAG: u8 = 0b0001_0000;
pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
pub const NEGATIVE_FLAG: u8 = 0b1000_0000;

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFF,
            pc: 0,
            status: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFF;
        self.pc = 0;
        self.status = 0;
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.status |= flag;
    }

    pub fn clear_flag(&mut self, flag: u8) {
        self.status &= !flag;
    }

    pub fn check_flag(&self, flag: u8) -> bool {
        self.status & flag != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_initialization() {
        let registers = Registers::new();
        assert_eq!(registers.a, 0);
        assert_eq!(registers.x, 0);
        assert_eq!(registers.y, 0);
        assert_eq!(registers.sp, 0xFF);
        assert_eq!(registers.pc, 0);
        assert_eq!(registers.status, 0);
    }

    #[test]
    fn test_register_reset() {
        let mut registers = Registers::new();
        registers.a = 10;
        registers.x = 20;
        registers.y = 30;
        registers.sp = 0x80;
        registers.pc = 0x1234;
        registers.status = 0xFF;

        registers.reset();
        assert_eq!(registers.a, 0);
        assert_eq!(registers.x, 0);
        assert_eq!(registers.y, 0);
        assert_eq!(registers.sp, 0xFF);
        assert_eq!(registers.pc, 0);
        assert_eq!(registers.status, 0);
    }

    #[test]
    fn test_status_flags() {
        let mut registers = Registers::new();

        // Test setting flags
        registers.set_flag(CARRY_FLAG);
        assert!(registers.check_flag(CARRY_FLAG));
        registers.set_flag(ZERO_FLAG);
        assert!(registers.check_flag(ZERO_FLAG));

        // Test clearing flags
        registers.clear_flag(CARRY_FLAG);
        assert!(!registers.check_flag(CARRY_FLAG));
        registers.clear_flag(ZERO_FLAG);
        assert!(!registers.check_flag(ZERO_FLAG));
    }
}