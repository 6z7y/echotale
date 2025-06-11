mod func;
mod loops;

use crate::{
    func::EchoSystem,
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
