mod ss;
use crate::ss::ascii_render::*;

fn main() {
    use terminal_size::{Width, Height, terminal_size};
    let size = terminal_size();
    let mut width;
    let mut height;
    if let Some((Width(w), Height(h))) = size {
        width = w;
        height = h;
    } else {
        width = 0;
        height = 0;
    }
    let mut ascii_render: Play = Play::new(width as usize, height as usize);
    ascii_render.run();
}
