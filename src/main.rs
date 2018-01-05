extern crate sdl2;

mod console;
mod m68k;
mod ram;
mod rom;
mod opcodes;
mod ui;

use console::Console;
use ui::UI;

// fn main() {
//     let mut console = Console::new("res/s2.bin").unwrap();
//     let mut console = Console::new("res/asmblr/test.bin").unwrap();
//     console.start();
//     console.step(); ??
// }

fn main() {
    let ttf_context = sdl2::ttf::init().unwrap();
    let ui = UI::new(&ttf_context);


    // loop {}
    // ui::render(&console);
}
