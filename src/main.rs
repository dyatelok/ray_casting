mod ss;
use crate::ss::ascii_render::*;

fn main() {
    let mut ascii_render: Play = Play::new(132, 42);
    ascii_render.run();
}
