
use crate::databus::DataBus;
use crate::registers::Registers;



pub struct CPU<M: DataBus> {
    registers: Registers,
    memory: M,
}

impl<M: DataBus> CPU<M> {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            memory: M::new(),
        }
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.memory.reset();
        // Additional reset logic if needed
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.memory.get_byte(self.registers.pc);
        self.registers.pc += 1;
        // Decode and execute the opcode
        // ...
    }
}
