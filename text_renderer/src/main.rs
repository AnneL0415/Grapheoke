use std::error::Error;
use std::fmt::Write;

use ttf_parser as ttf;

const FONT_PATH: &'static str = "Open_Sans/static/OpenSans-Regular.ttf";

struct Builder {
    cmds: String,
    current_point: (f32, f32),
}

impl ttf::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("Move to {} {}", x, y);
        write!(&mut self.cmds, "M {} {}", x, y).unwrap();

        self.current_point = (x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("Line to {} {}", x, y);
        write!(&mut self.cmds, "L {} {}", x, y).unwrap();

        // Print the function
        let x_t_offset = x - self.current_point.0;
        let y_t_offset = y - self.current_point.1;

        println!(
            "\\left({}+{}t,\\ {}+{}t\\right)",
            self.current_point.0, x_t_offset, self.current_point.1, y_t_offset
        );

        self.current_point = (x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!("Quad to {} {} {} {}", x1, y1, x, y);
        write!(&mut self.cmds, "Q {} {} {} {}", x1, y1, x, y).unwrap();

        // Print the function

        // B_x(t) = cur_x(1-t)^2 + 2x_1(t-1) + xt^2
        // B_y(t) = cur_y(1-t)^2 + 2y_1(t-1) + yt^2

        let b_x = format!(
            "{}*\\left(1-t\\right)^{{2}} + 2*{}*\\left(1-t\\right)t + {}*t^2",
            self.current_point.0, x1, x
        );
        let b_y = format!(
            "{}*\\left(1-t\\right)^{{2}} + 2*{}*\\left(1-t\\right)t + {}*t^2",
            self.current_point.1, y1, y
        );

        println!("\\left({}, {}\\right)", b_x, b_y);

        self.current_point = (x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!("Curve to {} {} {} {} {} {}", x1, y1, x2, y2, x, y);
        write!(&mut self.cmds, "C {} {} {} {} {} {}", x1, y1, x2, y2, x, y).unwrap();

        self.current_point = (x, y);
    }

    fn close(&mut self) {
        println!("Close");
        write!(&mut self.cmds, "Z").unwrap()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font_data = std::fs::read(FONT_PATH)?;
    let mut font_face = ttf::Face::parse(&font_data, 0)?;

    if let Some(a_character) = font_face.glyph_index('A') {
        println!("{:?}", a_character);
        // Check what kinds of information this has
        println!("{:?}", font_face.glyph_svg_image(a_character));
        println!("{:?}", font_face.glyph_raster_image(a_character, 100));
        let mut builder = Builder {
            cmds: String::new(),
            current_point: (0.0, 0.0),
        };
        println!("{:?}", font_face.outline_glyph(a_character, &mut builder));
        println!("Commands are: {}", builder.cmds);
    }
    Ok(())
}
