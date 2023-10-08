use std::error::Error;
use std::fmt::Write;
use std::path::PathBuf;

use pyo3::prelude::*;

use pyo3::types::PyList;
use ttf_parser as ttf;

mod lrc_parser;
mod renderer;

use lrc_parser::lrc_to_timings;
use renderer::generate_sentence_fns;

const FONT_PATH: &'static str = "Open_Sans/static/OpenSans-Regular.ttf";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font_data = std::fs::read(FONT_PATH)?;
    let mut font_face = ttf::Face::parse(&font_data, 0)?;

    let sentence = "The quick brown fox jumped over the lazy dog";
    // let fns = generate_sentence_fns(&mut font_face, sentence, 0.01, (0., 0.));
    let fns: Vec<String> = vec![];
    if let Some(lrc_file) = std::env::args().nth(1) {
        let lrc_path = PathBuf::from(lrc_file);
        let fns = lrc_to_timings(&mut font_face, 0.001, (0., 0.), lrc_path)?;
        println!("{:?}", fns);

        let code = std::fs::read_to_string("../desmos_api/graph_writer.py")?;
        Python::with_gil(|py| {
            let graph_writer =
                PyModule::from_code(py, &code, "graph_writer.py", "graph_writer").unwrap();
            // let function_list = builder.function_list.into_py_list(py).unwrap(); graph_writer.graph_fn(builder.function_list);
            let fns_py = PyList::new(py, fns);
            let args = (fns_py,);
            let graph_fn = graph_writer.getattr("graph_fn").unwrap();
            graph_fn.call(args, None).expect("Graph generation failed");
        });
        println!("Wrote graph to file");
    } else {
        println!("Usage: ./text_renderer [lrc-file]")
    }

    Ok(())
}
