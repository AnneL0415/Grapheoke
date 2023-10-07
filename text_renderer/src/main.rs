use std::error::Error;
use std::fmt::Write;

use pyo3::prelude::*;

use pyo3::types::PyList;
use ttf_parser as ttf;

const FONT_PATH: &'static str = "Open_Sans/static/OpenSans-Regular.ttf";

struct Builder {
    cmds: String,
    function_list: Vec<String>,
    current_point: (f32, f32),
    scale: f32,
    offset: (f32, f32),
}

impl Builder {
    fn new(scale: f32, offset: (f32, f32)) -> Self {
        Self {
            scale,
            offset,
            function_list: vec![],
            cmds: String::new(),
            current_point: (0.0, 0.0),
        }
    }
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
        let x_t_slope = x - self.current_point.0;
        let y_t_slope = y - self.current_point.1;

        let l_x_t = format!("{} + {}t", self.current_point.0, x_t_slope);
        let l_y_t = format!("{} + {}t", self.current_point.1, y_t_slope);

        self.function_list.push(format!(
            "\\left({} + {},\\ {} + {}\\right)",
            l_x_t, self.offset.0, l_y_t, self.offset.1
        ));

        self.current_point = (x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!("Quad to {} {} {} {}", x1, y1, x, y);
        write!(&mut self.cmds, "Q {} {} {} {}", x1, y1, x, y).unwrap();

        // Print the function
        // Quadratic Bézier curve with one intermediate format

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

        self.function_list.push(format!(
            "\\left({} + {}, {} + {}\\right)",
            b_x, self.offset.0, b_y, self.offset.1
        ));

        self.current_point = (x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!("Curve to {} {} {} {} {} {}", x1, y1, x2, y2, x, y);
        write!(&mut self.cmds, "C {} {} {} {} {} {}", x1, y1, x2, y2, x, y).unwrap();

        // Push to function list
        todo!();

        self.current_point = (x, y);
    }

    fn close(&mut self) {
        println!("Close");
        write!(&mut self.cmds, "Z").unwrap()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font_data = std::fs::read(FONT_PATH)?;
    let font_face = ttf::Face::parse(&font_data, 0)?;

    if let Some(a_character) = font_face.glyph_index('F') {
        println!("{:?}", a_character);
        // Check what kinds of information this has
        println!("{:?}", font_face.glyph_svg_image(a_character));
        println!("{:?}", font_face.glyph_raster_image(a_character, 100));
        let mut builder = Builder::new(0.1, (100.0, 50.0));
        println!("{:?}", font_face.outline_glyph(a_character, &mut builder));
        println!("Commands are: {}", builder.cmds);
        println!("Function array is: {:?}", builder.function_list);

        let code = std::fs::read_to_string("../desmos_api/graph_writer.py")?;
        Python::with_gil(|py| {
            let graph_writer =
                PyModule::from_code(py, &code, "graph_writer.py", "graph_writer").unwrap();
            // let function_list = builder.function_list.into_py_list(py).unwrap(); graph_writer.graph_fn(builder.function_list);
            let fns_py = PyList::new(py, builder.function_list);
            let args = (fns_py,);
            let graph_fn = graph_writer.getattr("graph_fn").unwrap();
            graph_fn.call(args, None).expect("Graph generation failed");
        })
    }
    Ok(())
}
