use std::error::Error;

use html_escape::encode_safe_to_writer;
use lol_html::{element, HtmlRewriter, Settings, OutputSink};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.html");
    println!("\n## input\n\n{input}");

    let mut output = vec![];
    let rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("img", |el| {
                el.set_attribute("loading", "lazy")?;
                Ok(())
            })],
            ..Default::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );
    process(input, rewriter)?;
    println!("\n## output\n\n{}", std::str::from_utf8(&output).unwrap());

    let input = std::str::from_utf8(&output[..])?;
    let mut output = vec![];
    let escaper = Escaper{ output: &mut output };
    process(input, escaper)?;
    println!("\n## output\n\n{}", std::str::from_utf8(&output).unwrap());

    Ok(())
}

enum ProcessorType{
    LazyLoading,
    HtmlEscape
}

impl ProcessorType {
    fn build<W: std::io::Write>(&self, output: W) -> dyn Processor {
        match self {
            ProcessorType::LazyLoading => {
                HtmlRewriter::new(
                    Settings {
                        element_content_handlers: vec![element!("img", |el| {
                            el.set_attribute("loading", "lazy")?;
                            Ok(())
                        })],
                        ..Default::default()
                    },
                    |c: &[u8]| output.write_all(c),
                )
            },
            ProcessorType::HtmlEscape => {
                Escaper{ output: &mut output }
            },
        }
    }
}

trait Processor: Sized{
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>>;
    fn end(self) -> Result<(), Box<dyn Error>>;
}

// Code will be simplified if input here is &[u8]
// But then printlns will all need to be updated
fn process<P: Processor>(input: &str, mut processor: P) -> Result<(), Box<dyn Error>>{
    processor.write(input.as_bytes())?;
    processor.end()?;
    Ok(())
}

impl<'processor, Output: OutputSink> Processor for HtmlRewriter<'processor, Output>{
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>> {
        HtmlRewriter::write(self, chunk).map_err(Into::into)
    }

    fn end(self) -> Result<(), Box<dyn Error>> {
        HtmlRewriter::end(self).map_err(Into::into)
    }
}

impl<Write: std::io::Write> Processor for Escaper<Write> {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>> {
        encode_safe_to_writer(std::str::from_utf8(chunk)?, &mut self.output).map_err(Into::into)
    }

    fn end(self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct Escaper<Write: std::io::Write> {
    output: Write
}