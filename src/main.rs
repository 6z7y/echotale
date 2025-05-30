mod structfn;
mod log;
mod loops;

use crate::{
    structfn::EchoSystem,
    loops::loops
};

fn main() -> std::io::Result<()> {
    EchoSystem::draw_ascii();
    println!("Wait a moment");
    EchoSystem::sleeep(2500);

    loops()?;

    print!("file path: /tmp/echotale_history.txt");
    Ok(())
}
