use lrc::Lyrics;
use std::path::PathBuf;
use ttf_parser as ttf;

use crate::renderer::generate_sentence_fns;

pub fn lrc_to_timings(
    face: &mut ttf::Face,
    scale: f32,
    offset: (f32, f32),
    lrc_file: PathBuf,
) -> Result<Vec<(i64, Vec<String>)>, Box<dyn std::error::Error>> {
    let lrc = std::fs::read_to_string(lrc_file)?;
    let parsed_lrc = Lyrics::from_str(lrc)?;
    let mut output = vec![];
    let mut prev_timestamp: i64 = 0;
    let timed_lines = parsed_lrc.get_timed_lines();

    // Supposedly some LRC files have nothing on the last tag, highlighting that it's the end of the song.
    // So we can skip the last one, since each i actually inserts the lyrics for the
    // i - 1th entry
    for i in 0..timed_lines.len() {
        // time is in milliseconds
        let line = timed_lines[i].1.clone();
        let line_fns = generate_sentence_fns(face, &line, scale, offset);
        output.push((timed_lines[i].0.get_timestamp() - prev_timestamp, line_fns));
        prev_timestamp = timed_lines[i].0.get_timestamp();
    }
    Ok(output)
}
