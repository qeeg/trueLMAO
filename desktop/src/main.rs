use fltk::{
    app,
    draw,
    button::Button,
    frame::Frame,
    input::IntInput,
    prelude::*,
    window::Window,
    text::{TextBuffer, TextDisplay},
};

use emu::Megadrive;

#[derive(Debug, Copy, Clone)]
pub enum Update {
    Step, Frame, Toggle
}

fn main() {
    let app = app::App::default();
    let buf: Vec<u8> = include_bytes!("./s1.bin").to_vec();

    let mut emu = Megadrive::new(buf);

    let mut wind = Window::new(100, 100, 800, 600, "trueLMAO");
    let mut toggle = Button::new(400, 300, 80, 40, "stop");
    let mut but = Button::new(400, 350, 80, 40, "frame");
    let mut step = Button::new(400, 400, 80, 40, "step");
    let mut stepby = IntInput::new(400, 450, 80, 40, "step by");
    let mut info = TextDisplay::new(0, 0, 500, 300, "asm");
    stepby.set_value("1");

    let mut pal = Frame::new(0, 300, 16, 4, "");
    let mut palbuf: Vec<u8> = vec![0xFF; (16 * 4 * 4) as usize];

    let mut vram = Frame::new(500, 0, 256, 513, "");
    let mut vrambuf: Vec<u8> = vec![0xFF; (256 * 513 * 4) as usize];

    let mut screen = Frame::new(0, 350, 320, 240, "");
    let mut screenbuf: Vec<u8> = vec![0xFF; (320 * 240 * 4) as usize];

    wind.end();
    wind.show();

    unsafe {
        draw::draw_rgba_nocopy(&mut pal, &palbuf);
        draw::draw_rgba_nocopy(&mut vram, &vrambuf);
        draw::draw_rgba_nocopy(&mut screen, &screenbuf);
    }

    pal.set_size(160,40);

    info.set_buffer(TextBuffer::default());
    let mut buffer = info.buffer().unwrap();

    let (s, r) = app::channel::<Update>();

    but.set_callback(move |_| {
        s.send(Update::Frame);
    });

    step.set_callback(move |_| {
        s.send(Update::Step);
    });

    toggle.set_callback(move |_| {
        s.send(Update::Toggle);
    });

    let name = emu.core.mem.rom.domestic_name()
        .split_whitespace().collect::<Vec<&str>>().join(" ");

    wind.set_label(&format!("trueLMAO - {}", name));

    let mut running = true;

    while app.wait() {
        while let Some(msg) = r.recv() {
            println!("{:#?}", "asd");
            match msg {
                Update::Step => {
                    emu.step_n(stepby.value().parse::<usize>().unwrap_or(1));
                },
                Update::Frame => {
                    emu.frame();
                },
                Update::Toggle => {
                    running = !running;
                    println!("{:?}", running);
                    toggle.set_label(if running { "stop" } else { "go" });
                },
            }
        }

        if running {
            emu.frame();

        }
        let mut debug = String::new();
        debug.push_str(&format!("PC: {:X}\n\n", emu.core.pc));
        // let v = emu.core.mem.vdp.VRAM.iter().map(|x|format!("{:X}", x)).collect::<Vec<String>>().join(" ");
        // debug.push_str(&format!("VRAM: {}\n\n", v));
        debug.push_str(&format!("D "));
        for i in 0..=7 {
            debug.push_str(&format!("{:X} ", emu.core.dar[i]));
        }
        debug.push_str(&format!("\n"));

        debug.push_str(&format!("A "));
        for i in 0..=7 {
            debug.push_str(&format!("{:X} ", emu.core.dar[i + 8]));
        }
        debug.push_str(&format!("\n"));
        debug.push_str(&format!("\n"));

        for (pc, opcode) in emu.disasm() {
            debug.push_str(&format!("0x{:X}\t{}\n", pc, opcode));
        }
        buffer.set_text(&debug);

        // render CRAM

        let cram_rgb = emu.core.mem.vdp.cram_rgb();

        for (i, (red, green, blue)) in cram_rgb.iter().enumerate() {
            let index = i * 4;
            palbuf[index] = *red;
            palbuf[index+1] = *green;
            palbuf[index+2] = *blue;
        }

        // render VRAM

        let bg_color = emu.core.mem.vdp.bg_color();

        // clear screen
        for pixel in screenbuf.chunks_mut(4) {
            pixel[0] = bg_color.0;
            pixel[1] = bg_color.1;
            pixel[2] = bg_color.2;
        };

        for (i, duxels) in emu.core.mem.vdp.VRAM.chunks(32).enumerate() {
            let x_base = (i % 32) * 4 * 8;
            let y_base = (i / 32) * 4 * 8 * 256;
            let mut x = 0;
            let mut y = 0;

            let mut tile = vec![];
            for duxel in duxels {
                let first = (*duxel & 0xF0) >> 4;
                let second = *duxel & 0xF;
                tile.push(first);
                tile.push(second);
            }

            for pixel in tile {
                let (r, g, b) = cram_rgb[pixel as usize];
                let base = x_base + y_base + x + y;

                if base+2 > vrambuf.len() {
                    break;
                }

                vrambuf[base] = r;
                vrambuf[base+1] = g;
                vrambuf[base+2] = b;
                x += 4;
                if x >= (8 * 4) {
                    x = 0;
                    y += 256 * 4;
                }
            }
        }


        wind.redraw();

        app::sleep(0.016);
    }
}
