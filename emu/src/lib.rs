use r68k_emu::cpu::{STACK_POINTER_REG, ConfiguredCore};
use r68k_emu::interrupts::AutoInterruptController;

mod io;
mod mem;
mod rom;
mod vdp;
mod z80;

pub struct Megadrive {
    pub core: ConfiguredCore<AutoInterruptController, mem::Mem>,
    pub screen: [u8; 320 * 240 * 3],
    // version: NTSC/PAL
}

impl Megadrive {
    pub fn new(buf: Vec<u8>) -> Self {
        let mem = mem::Mem::new(buf.into());

        let int_ctrl = AutoInterruptController::new();
        let mut core = ConfiguredCore::new_with(mem.rom.entry_point(), int_ctrl, mem);

        core.dar[STACK_POINTER_REG] = core.mem.rom.stack_pointer();

        Megadrive {
            core,
            screen: [0; 320 * 240 * 3],
        }
    }

    pub fn step_n(&mut self, amount: usize) {
        for _ in 0..amount {
            self.core.execute1();
        }
    }

    pub fn disasm(&self) -> Vec<(u32, String)> {
        use r68k_tools::PC;
        let mut buffer = Vec::new();
        let mut opcodes = Vec::new();
        // longest opcode is 16 bytes
        for i in 0..(16 * 10) {
            buffer.push(self.core.mem.rom.read_byte(self.core.pc + i));
        }
        let mvec = r68k_tools::memory::MemoryVec::new8(PC(0), buffer);
        let mut cursor = PC(0);
        for _ in 0..10 {
            let disasm = r68k_tools::disassembler::disassemble(cursor, &mvec);
            if let Ok((pc, opcode)) = disasm {
                opcodes.push((cursor.0 + self.core.pc, opcode.to_string().to_lowercase()));
                cursor = pc;
            }
        }

        opcodes
    }

    pub fn frame(&mut self) {
        // TODO: read controller
        /* cycle counts initially taken from drx/kiwi */
        // TODO: use a counter instead

        self.clear_screen();

        self.core.mem.vdp.unset_status(vdp::VBLANK_MASK);
        self.core.mem.vdp.unset_status(vdp::VINT_MASK);

        let screen_height = self.core.mem.vdp.screen_height();
        let mut hint_counter = self.core.mem.vdp.hint_counter();
        for screen_y in 0..screen_height {
            self.core.execute(2680);

            hint_counter -= 1;
            if hint_counter < 0 {
                hint_counter = self.core.mem.vdp.hint_counter();

                if self.core.mem.vdp.hint_enabled() {
                    self.core.int_ctrl.request_interrupt(4);
                }

            }

            self.core.mem.vdp.set_status(vdp::HBLANK_MASK);
            self.core.execute(636);
            self.core.mem.vdp.unset_status(vdp::HBLANK_MASK);

            self.core.execute(104);

            self.fire_beam(screen_y);

        }

        self.core.mem.vdp.set_status(vdp::VBLANK_MASK);

        self.core.execute(588);

        if self.core.mem.vdp.vint_enabled() {
            self.core.int_ctrl.request_interrupt(6);
            self.core.mem.vdp.set_status(vdp::VINT_MASK);
        }

        self.core.execute(3420-588);

        for _ in screen_height..262 {
            self.core.execute(3420);
        }
    }

    fn clear_screen(&mut self) {
        let bg_color = self.core.mem.vdp.bg_color();
        for pixel in self.screen.chunks_mut(3) {
            pixel[0] = bg_color.0;
            pixel[1] = bg_color.1;
            pixel[2] = bg_color.2;
        };
    }

    fn fire_beam(&mut self, screen_y: usize) {
        let (cell_w, cell_h) = self.core.mem.vdp.scroll_size();
        let (plane_a, plane_b) = self.core.mem.vdp.nametables();
        let (hscroll_a, hscroll_b) = self.core.mem.vdp.hscroll(screen_y);
        let screen_width = self.core.mem.vdp.screen_width();

        let sprites = self.core.mem.vdp.sprites(screen_y);

        for screen_x in 0..screen_width {
            let (vscroll_a, vscroll_b) = self.core.mem.vdp.vscroll(screen_x);


            self.draw_plane_pixel(
                cell_w,
                cell_h,
                screen_x,
                screen_y,
                screen_width,
                plane_b,
                hscroll_b,
                vscroll_b,
            );

            // p1

            self.draw_plane_pixel(
                cell_w,
                cell_h,
                screen_x,
                screen_y,
                screen_width,
                plane_a,
                hscroll_a,
                vscroll_a,
            );

            // p2
        }

        // TODO: draw_plane_line


        // if sprites.len() > 0 {
        //     println!("{:#?}", sprites[0]);
        // }
        for sprite in sprites {
            let crate::vdp::Sprite { x_pos, width, .. } = sprite;

            for i in 0..width * 8 {
                let x_offset = x_pos + i;

                let (r, g, b) = self.core.mem.vdp.color(0, 4);

                let screen_offset = (x_offset + (screen_y * screen_width)) * 3;

                self.screen[screen_offset] = r;
                self.screen[screen_offset + 1] = g;
                self.screen[screen_offset + 2] = b;
            }

            // break;
        }
    }

    fn draw_plane_pixel(
        &mut self,
        cell_w: usize,
        cell_h: usize,
        screen_x: usize,
        screen_y: usize,
        screen_width: usize,
        nametable: usize,
        hscroll: usize,
        vscroll: usize
    ) {
        let plane_width = cell_w * 8;
        let plane_height = cell_h * 8;

        let hscroll_rem = hscroll % plane_width;
        let x_offset = (screen_x + plane_width - hscroll_rem) % plane_width;
        let y_offset = (screen_y + vscroll) % plane_height;

        let tile_index = ((x_offset / 8) + (y_offset / 8 * cell_w)) * 2;
        let tile_slice = &self.core.mem.vdp.VRAM[nametable + tile_index..];

        let word = (tile_slice[0] as usize) << 8 | tile_slice[1] as usize;
        let byte = word >> 8;

        let priority = byte >> 8;
        let tile = word & 0x7FF;
        let vflip = (byte & 0x10) != 0;
        let hflip = (byte & 0x8) != 0;
        let palette = (byte & 0x60) >> 5;

        let hline = if hflip { x_offset ^ 0xF } else { x_offset };
        let x_offset = (hline & 6) >> 1;
        let vline = y_offset & 7;
        let y_offset = if vflip { vline ^ 7 } else { vline } * 4;

        let px = self.core.mem.vdp.VRAM[(tile * 32) + x_offset + y_offset];
        let px = if hline & 1 == 0 { px >> 4 } else { px & 0xF };

        if px != 0 {
            let (r, g, b) = self.core.mem.vdp.color(palette, px as _);

            let screen_offset = (screen_x + (screen_y * screen_width)) * 3;

            self.screen[screen_offset] = r;
            self.screen[screen_offset + 1] = g;
            self.screen[screen_offset + 2] = b;
        }
    }
}
