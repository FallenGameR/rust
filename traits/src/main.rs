use std::error::Error;

use lol_html::{element, HtmlRewriter, Settings};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.html");
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::try_new(
        Settings {
            element_content_handlers: vec![element!("img", |el| {
                el.set_attribute("loading", "lazy")?;
                Ok(())
            })],
            ..Default::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    )?;
    rewriter.write(input.as_bytes())?;
    rewriter.end()?;

    println!("input: {input}");
    println!("output: {}", std::str::from_utf8(&output).unwrap());

    Ok(())
}
