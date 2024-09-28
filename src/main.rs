mod ansi;
mod bind;
mod buffer;
mod canvas;
mod color;
mod control;
mod display;
mod editor;
mod error;
mod grid;
mod io;
mod key;
mod op;
mod opt;
mod term;
mod window;
mod workspace;

use crate::bind::BindingMap;
use crate::buffer::Buffer;
use crate::control::Controller;
use crate::display::{Point, Size};
use crate::editor::Editor;
use crate::error::Result;
use crate::key::Keyboard;
use crate::opt::Options;
use crate::workspace::{Placement, Workspace};

use std::env;
use std::ops::Drop;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            println!("{e}");
            ExitCode::from(1)
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

struct Reset;

impl Drop for Reset {
    fn drop(&mut self) {
        if let Err(e) = term::restore() {
            println!("error resetting terminal: {e:?}");
        }
    }
}

fn run() -> Result<()> {
    let opts = Options::parse(env::args().skip(1))?;
    if opts.help {
        println!("usage: ped");
    } else {
        let bindings = BindingMap::new();

        let mut buffer = Buffer::new();
        if let Some(file) = opts.files.iter().next() {
            let _ = io::read_file(file, &mut buffer)?;
            buffer.set_pos(0);
        };

        let (rows, cols) = term::size()?;
        term::init()?;
        let _reset = Reset;
        let keyboard = Keyboard::new();
        let mut workspace = Workspace::new(Point::ORIGIN, Size::new(rows, cols))?;
        let _ = workspace.add_view(Placement::Top);
        let window = workspace.views().next().unwrap().window().clone();
        let editor = Editor::new(Buffer::to_ref(buffer), window);
        let mut controller = Controller::new(keyboard, editor, bindings);
        controller.run()?
    }
    Ok(())
}
