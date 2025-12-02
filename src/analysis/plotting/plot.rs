use std::slice;

// functions which build plots using plotters
use plotters::prelude::*;
// use plotters::
// plotters::element::pie::{Pie};
pub type Slice<'a> = (&'a str, RGBColor, f64);
fn slice_unzip<'a>(slices: &'a [Slice]) -> (Vec<&'a str>, Vec<RGBColor>, Vec<f64>) {
    slices.iter().fold(
        (vec![], vec![], vec![]),
        |(mut labels, mut colors, mut sizes), (label, color, size)| {
            labels.push(label);
            colors.push(*color);
            sizes.push(*size);
            (labels, colors, sizes)
        },
    )
}

fn build_pie_chart(slices: &[Slice]) {
    // let pie = Pie::;
    let (labels, colors, sizes) = slice_unzip(slices);

    let pie = Pie::new(&(0, 0), &1.0, &sizes, &colors, &labels);
}

fn hexcolor_to_rgbcolor(s: &str) -> Result<RGBColor, String> {
    let (r, g, b) = {
        let string = match s.strip_prefix("#") {
            Some(new_s) => new_s.to_string(),
            None => s.to_string(),
        };
        if string.len() != 6 {
            Err(format!("{s} is not a valid hexidecimal color"))?
        } else {
            let bytes: Vec<u8> = string
                .chars()
                .collect::<Vec<char>>()
                .chunks_exact(2)
                .inspect(|c| eprintln!("c = {c:?} (should be a two element slice/str)"))
                .map(|chars| {
                    u8::from_str_radix(&chars.iter().collect::<String>(), 16)
                        .map_err(|e| format!("could not parse hex digit {chars:?}: {e}"))
                })
                .collect::<Result<Vec<u8>, String>>()?;
            println!("parsed color bytes {bytes:?} from {s}");
            (bytes[0], bytes[1], bytes[2])
        }
    };
    Ok(RGBColor(r, g, b))
}
