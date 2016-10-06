use util::get_bit;

/*   p. 1481 nice list of CP15 registers
     p. 1528 CCSIDR
     p. 1530 CLIDR
     p. 1556 CTR
     p. 1558 DACR
     p. 1752 DSB
     p. 1648 MIDR
     p. 1651 MPIDR
     p. 1707 SCTLR
     p. 1724 TTBCR
     p. 1729 TTBR0
     p. 1733 TTBR1
*/

const QEMU_MIDR : u32 = 0x410fc090;
const QEMU_MMFR0: u32 = 0x100103;
const QEMU_MPIDR: u32 = 0x20000000;
const QEMU_CLIDR: u32 = 0x9000003;
const QEMU_CCSIDR:u32 = 0xe00fe019;
const QEMU_CTR:   u32 = 0x80038003;
const QEMU_PMCR:  u32 = 0x41000000;

impl ::cpu::core::CPU {
    pub fn cp15_get_reg(&self, crn: u32, opc1: u32, crm: u32, opc2: u32) -> u32 {
        let tuple = (crn, opc1, crm, opc2);
        match tuple {
            (0, 0, 0, 1) => QEMU_CTR,
            (0, 1, 0, 0) => QEMU_CCSIDR,
            (0, 1, 0, 1) => QEMU_CLIDR,
            (0, 0, 0, 0) => QEMU_MIDR,
            (0, 0, 1, 4) => QEMU_MMFR0,
            (0, 0, 1, 5) => QEMU_MPIDR,
            (1, 0, 0, 0) => self.cp15_get_sctlr(),
            (9, 0, 12,0) => QEMU_PMCR,
            (5, 0, 0, 0) => unsafe { ::memory::dfsr },
            (6, 0, 0, 0) => unsafe { ::memory::dfar },
            (13,0, 0, 3) => self.thread_id,
            _ => {
                println!("({}, {}, {}, {})", crn, opc1, crm, opc2);
                unreachable!()},
        }
    }

    pub fn cp15_set_reg(&mut self, crn: u32, opc1: u32, rt: u32, crm: u32, opc2: u32) {
        let tuple = (crn, opc1, crm, opc2);
        match tuple {
            ( 0, 2, 0 , 0) => {}, // CSSELR
            ( 3, 0, 0 , 0) => self.cp15_set_dacr(rt),
            ( 7, 0, 14, 2) | // DCCISW
            ( 7, 0, 10, 5) | // DMB
            ( 7, 0, 10, 4) | // DSB
            ( 7, 0, 5 , 0) | // ICIALLU
            ( 7, 0, 5 , 6) | // BPIALL
            (10, 0, 2 , 0) | // PRRR
            (10, 0, 2 , 1) => {}, // NMRR
            ( 1, 0, 0 , 0) => self.cp15_set_sctlr(rt),
            ( 8, 0, 7 , 0) => {}, // TLBIALL
            ( 2, 0, 0 , 0) => self.cp15_set_ttbr0(rt),
            ( 2, 0, 0 , 1) => self.cp15_set_ttbr1(rt),
            ( 2, 0, 0 , 2) => self.cp15_set_ttbcr(rt),
            ( 7, 0, 5 , 4) | // ISB
            ( 7, 0, 10, 1) | // DCCMVAC
            ( 7, 0, 14, 1) | // DCCIMVAC
            ( 7, 0, 11, 1) | // DCCMVAU
            ( 7, 0, 5 , 1) => {}, // ICIMVAU
            (13, 0, 0 , 3) => self.cp15_set_thread_id(rt), // TPIDRURO
            ( 9, 0, 12, 0) | // PMCR
            ( 9, 0, 12, 2) | // PMCNTENCLR
            ( 9, 0, 14, 2) | // PMINTENCLR
            (13, 0, 0 , 1) | // CONTEXTIDR
            ( 8, 0, 7 , 1) | // TLBIMVA
            ( 8, 0, 7 , 2) => {}, // TLBIASID
            _ => {
                println!("({}, {}, {}, {})", crn, opc1, crm, opc2);
                unreachable!()},

        };
    }

    pub fn cp15_set_dacr(&mut self, dacr: u32) {
        self.dacr = dacr;
    }

    pub fn cp15_get_sctlr(&self) -> u32 {
        self.sctlr
    }

    pub fn cp15_set_sctlr(&mut self, sctlr: u32) {
        self.sctlr = sctlr;
        assert!(get_bit(sctlr, 30) == 0);
        assert!(get_bit(sctlr, 29) == 0); // access flag disabled
        assert!(get_bit(sctlr, 25) == 0);
        assert!(get_bit(sctlr, 24) == 0);
        assert!(get_bit(sctlr, 17) == 0); // hw access flag
        assert!(get_bit(sctlr, 7)  == 0);
        assert!(get_bit(sctlr, 1)  == 0);
        self.mem.mmu_on(get_bit(sctlr, 0) == 1);
    }

    pub fn cp15_set_ttbr0(&mut self, ttbr0: u32) {
        self.mem.set_ttbr0(ttbr0);
    }

    pub fn cp15_set_ttbr1(&mut self, ttbr1: u32) {
        self.mem.set_ttbr1(ttbr1);
    }

    pub fn cp15_set_ttbcr(&mut self, ttbcr: u32) {
        self.mem.set_ttbcr(ttbcr);
    }

    pub fn cp15_set_thread_id(&mut self, thread_id: u32) {
        self.thread_id = thread_id;
    }
}
