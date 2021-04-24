use r68k_emu::cpu::ConfiguredCore;
use r68k_emu::cpu::Core;
use r68k_emu::interrupts::AutoInterruptController;
use r68k_emu::ram::pagedmem::PagedMem;
use r68k_tools::PC;
use r68k_tools::memory::MemoryVec;
use r68k_tools::disassembler::disassemble;

mod mem;
mod rom;

pub struct Emulator {
    core: ConfiguredCore<AutoInterruptController, mem::Mem>,
}

impl Emulator {
    // cpu
    // rom
    // ram
    // vdp
    pub fn new() -> Self {

        let buf: Vec<u8> = include_bytes!("../../notes/res/s1.bin").to_vec();

        let mem = mem::Mem {
            rom: rom::Rom::from_vec(buf),
        };

        // orbtk/iced for proto ui
        // use a listing file

        let int_ctrl = AutoInterruptController::new();
        // let mut mem = PagedMem::new(0);
        // for (i, data) in buf.iter().enumerate() {
        //     mem.write_u8(i as u32, *data as u32);
        // }
        let mut core = ConfiguredCore::new_with(0x206, int_ctrl, mem);

        core.pc = core.mem.rom.entry_point();


    //// r68k.pc = 0x206;
    //// r68k.resume_normal_processing();
    //println!("PC is 0x{:06x}", r68k.pc);
    //let cycle = r68k.execute1();
    //println!("PC is 0x{:06x}", r68k.pc);


    ////
    //println!("{:?}", cycle);
    //let mem = MemoryVec::new8(PC(0), buf.clone());
    //let res = disassemble(PC(0x206), &mem);
    //println!("{}", res.unwrap().1);

        Emulator {
            core,
        }
    }

    pub fn step1(&mut self) {
        self.core.pc = 0x206;
        self.core.execute1();
    }

    pub fn disasm_stuff(&self) -> String {

        "test".to_string()
    }
}
