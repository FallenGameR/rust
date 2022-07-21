use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
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
fn test_parse_complex()
{
    assert_eq!(parse_complex("1.25,-0.635"), Some(Complex{ re: 1.25, im: -0.635 }));
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
