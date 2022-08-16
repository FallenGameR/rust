use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;
use rayon::prelude::*;
extern crate num_cpus;

/// cargo build --release
/// hyperfine ".\target\release\mandelbrot.exe mandel.png 4000x3000 -1.08,0.28 -1.03,0.23" --warmup 1
///
/// cargo run mandel.png 1000x750 -1.08,0.28 -1.03,0.23; start mandel.png
///
/// Single thread:
/// - SEKIREI   - 5.6 sec
/// - ALEXKO-11 - 4.2 sec x1.3
/// - ALEXKO-LS - 6.5 sec x0.8
///
/// Multi thread:
/// - SEKIREI   - 3.0 sec
/// - ALEXKO-11 - 0.5 sec x6
/// - ALEXKO-LS - 1.1 sec x2.7
///
/// Rayon mutithread
/// - ALEXKO-11 - 0.3 sec
/// - ALEXKO-LS - 0.7 sec
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} <file.png> <width>x<height> <upper_left_coordinate> <lower_right_coordinate>", args[0]);
        eprintln!("Example: {} mandel.png 4000x3000 -1.08,0.28 -1.03,0.23", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left dot");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right dot");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // Single threaded
    //render_single_thread(&mut pixels, bounds, upper_left, lower_right);

    // Multi threaded crossbeam - chunk per thread
    //render_multi_thread_crossbeam(&mut pixels, bounds, upper_left, lower_right);

    // Multi threaded rayon - line per thread
    render_multi_thread_rayon(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing output PNG file");
}

fn render_multi_thread_rayon(
    pixels: &mut[u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    let lines: Vec<(usize, &mut[u8])> = pixels
        .chunks_mut(bounds.0)
        .enumerate()
        .collect();

    lines.into_par_iter()
        .for_each(|(top, line)| {
            let line_bounds = (bounds.0, 1);
            let line_upper_left = convert_pixel_to_dot(bounds, (0, top), upper_left, lower_right);
            let line_lower_right = convert_pixel_to_dot(bounds, (bounds.0, top+1), upper_left, lower_right);
            render(line, line_bounds, line_upper_left, line_lower_right);
        });
}

fn render_multi_thread_crossbeam(
    pixels: &mut[u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    let threads = num_cpus::get();
    println!("Threads used: {}", threads);
    let rows_in_part = bounds.1 / threads + 1;

    {
        let parts: Vec<&mut [u8]> = pixels.chunks_mut(rows_in_part * bounds.0).collect();
        crossbeam::scope(|thread_spawner| {
            for (i, part) in parts.into_iter().enumerate() {
                let top = rows_in_part * i;
                let height = part.len() / bounds.0;
                let part_bounds = (bounds.0, height);
                let part_upper_left = convert_pixel_to_dot(bounds, (0, top), upper_left, lower_right);
                let part_lower_right = convert_pixel_to_dot(bounds, (bounds.0, top + height), upper_left, lower_right);
                thread_spawner.spawn(move |_| {
                    render(part, part_bounds, part_upper_left, part_lower_right);
                });
            }

        }).unwrap();
    }
}

fn render_single_thread(
    pixels: &mut[u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
        render(pixels, bounds, upper_left, lower_right);
}

/// dimensions of pixcure are given by bounds
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    assert!(pixels.len() == bounds.0 * bounds.1);

    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

/// Render viewport of mandelbrot set into 255 grayscale
fn render(
    pixels: &mut [u8],
    pixel_frame_col_row: (usize, usize),
    dot_left_upper: Complex<f64>,
    dot_right_lower: Complex<f64>,
) {
    assert!(pixels.len() == pixel_frame_col_row.0 * pixel_frame_col_row.1);

    for row in 0..pixel_frame_col_row.1 {
        for col in 0..pixel_frame_col_row.0 {
            let dot = convert_pixel_to_dot(
                pixel_frame_col_row, (col, row), dot_left_upper, dot_right_lower);
            pixels[row * pixel_frame_col_row.0 + col] =
                match escape_time(dot, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
        }
    }
}

/// converts from pixel space to dot space knowing boundary boxes for both
fn convert_pixel_to_dot(
    pixel_frame_col_row: (usize, usize),
    pixel_col_row: (usize, usize),
    dot_left_upper: Complex<f64>,
    dot_right_lower: Complex<f64>,
) -> Complex<f64> {
    let dot_frame_width = dot_right_lower.re - dot_left_upper.re;
    let dot_frame_height = dot_left_upper.im - dot_right_lower.im;
    let dot_re_relative =
        pixel_col_row.0 as f64 * dot_frame_width / pixel_frame_col_row.0 as f64;
    let dot_im_relative =
        pixel_col_row.1 as f64 * dot_frame_height / pixel_frame_col_row.1 as f64;

    Complex {
        re: dot_left_upper.re + dot_re_relative,
        im: dot_left_upper.im - dot_im_relative,
    }
}

#[test]
fn test_convert_pixel_to_dot() {
    assert_eq!(
        convert_pixel_to_dot(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// Parses pairs of T separated by separator char.
/// None if could not parse
/// (T,T) if parsing was successful
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(left), Ok(right)) => Some((left, right)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20zz", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x0.6", 'x'), Some((0.5, 0.6)));
}

/// parse complex number like 1.2,4.5 from a string
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.635"),
        Some(Complex {
            re: 1.25,
            im: -0.635
        })
    );
    assert_eq!(parse_complex(",1"), None);
}

/// None when after limit iterations we still think c is in Mandelbrot set
/// (it doesn't go into infinity). Otherwise return iteration number when we
/// found out that it is not in the set (it stays farther away than circle
/// with radius 2 - it is proven that such points will move into infinity
/// on later iterations)
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}
