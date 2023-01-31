use std::error::Error;

use html_escape::encode_safe_to_writer;
use lol_html::{element, HtmlRewriter, Settings, OutputSink};

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
    process(input, &mut rewriter)?;

    println!("input: {input}");
    println!("output: {}", std::str::from_utf8(&output).unwrap());

    let input = std::str::from_utf8(&output[..])?;
    let mut output = vec![];
    let mut escaper = Escaper{ output: &mut output };
    process(input, &mut escaper)?;

    println!("input: {input}");
    println!("output: {}", std::str::from_utf8(&output).unwrap());

    Ok(())
}

trait Processor{
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>>;
    fn end(&mut self) -> Result<(), Box<dyn Error>>;
}

impl<'processor, Output: OutputSink> Processor for HtmlRewriter<'processor, Output>{
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>> {
        HtmlRewriter::write(self, chunk).map_err(Into::into)
    }

    fn end(&mut self) -> Result<(), Box<dyn Error>> {
        HtmlRewriter::end(self).map_err(Into::into)
    }
}

fn process(input: &str, processor: &mut dyn Processor) -> Result<(), Box<dyn Error>>{
    processor.write(input.as_bytes())?;
    processor.end()?;
    Ok(())
}

struct Escaper<Write: std::io::Write> {
    output: Write
}

impl<Write: std::io::Write> Processor for Escaper<Write> {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>> {
        encode_safe_to_writer(std::str::from_utf8(chunk)?, &mut self.output).map_err(Into::into)
    }

    fn end(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}