/*   p. 2124 list of CP14 registers
     p. 2231 DBGDIDR
*/

const QEMU_DBGDIDR : u32 = 0x35141000;

impl ::cpu::core::CPU {
    pub fn cp14_get_reg(&self, crn: u32, opc1: u32, crm: u32, opc2: u32) -> u32 {
        let tuple = (crn, opc1, crm, opc2);
        match tuple {
            (0, 0, 0, 0) => QEMU_DBGDIDR,
            _ => { 
                println!("({}, {}, {}, {})", crn, opc1, crm, opc2);
                unreachable!()},
        }
    }
}
