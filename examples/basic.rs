use std::fmt::Write;
use std::time::Duration;

pub fn main() {
    vita_dbgscreen::set_dbg_screen_panic_handler();
    let mut screen = vita_dbgscreen::DebugScreen::new();
    writeln!(screen, "This not-so-bare-metal is starting to rust!").ok();
    std::thread::sleep(Duration::from_secs(2));
    writeln!(screen, "See? Told ya!").ok();
    std::thread::sleep(Duration::from_secs(5));
}
