use std::{env, ops::Range, path::PathBuf, slice};

// functions which build plots using plotters
use plotters::{element::Drawable, prelude::*};

use crate::database::epoch::LouisEpoch;
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

fn pie_chart<'a>(slices: &'a [Slice]) -> Result<(), String> {
    // let pie = Pie::;
    let (labels, colors, sizes) = slice_unzip(slices);
    let plot_path = get_plot_path();
    let root = BitMapBackend::new(&plot_path, PLOT_DIMENSIONS)
        .into_drawing_area()
        .titled("Pie chart title", ("sans-serif", 40).into_font())
        .map_err(|e| format!("could not draw title: {e}"))?;
    root.fill(&WHITE);
    let pie = Pie::new(&(0, 0), &1.0, &sizes, &colors, &labels);
    root.draw(&pie);
    root.present()
        .map_err(|e| format!("failed to present plot: {e}"))?;
    Ok(())
}
// change to UnixEpoch once i can figure out how to translate it into the desired value. or just translate before feeding. internally it is recast as usize.
type Line<'a> = (&'a str, RGBColor, Vec<(LouisEpoch, usize)>);
//
fn line_chart<'a>(
    lines: &'a [Line],
    x_range: Range<usize>,
    y_range: Range<usize>,
) -> Result<(), String> {
    let plot_path = get_plot_path();
    let root = BitMapBackend::new(&plot_path, PLOT_DIMENSIONS).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .caption("Line chart caption", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range, y_range)
        .map_err(|e| format!("could not build chart: {e}"))?;
    for (name, color, values) in lines {
        chart
            .draw_series(LineSeries::new(
                values.iter().map(|(time, y)| (*time as usize, *y)),
                color,
            ))
            .map_err(|e| format!("could not draw series for {name}: {e}"))?
            .label(*name);
    }
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .map_err(|e| format!("failed to set background: {e}"))?;
    root.present()
        .map_err(|e| format!("failed to present plot: {e}"))?;
    Ok(())
}
const PLOT_DIMENSIONS: (u32, u32) = (100, 100);
// fn build_backend<'a>() -> SVGBackend<'a> {

//     // SVGBackend::new(&get_plot_path())
// }
fn get_plot_path() -> PathBuf {
    // let path = if cfg!(windows){
    //     PathBuf::from()
    // }else{
    //     PathBuf::from("/tmp").join("louisbot_plot.svg")
    // };
    let tmp = if cfg!(windows) {
        env::var("TEMP").unwrap()
    } else {
        env::var("TMP").unwrap_or("/tmp".to_string())
    };
    PathBuf::from(tmp).join("louisbot_plot.svg")
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
