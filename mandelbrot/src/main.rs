use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
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
