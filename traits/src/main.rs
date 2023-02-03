use std::error::Error;

use html_escape::encode_safe_to_writer;
use lol_html::{element, HtmlRewriter, OutputSink, Settings};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.html");
    println!("\n## input\n\n{input}");

    let mut output = vec![];
    let rewriter = ProcessorType::LazyLoading.build(&mut output);
    process(input, rewriter)?;
    println!("\n## output\n\n{}", std::str::from_utf8(&output).unwrap());

    let input = std::str::from_utf8(&output[..])?;
    let mut output = vec![];
    let escaper = ProcessorType::HtmlEscape.build(&mut output);
    process(input, escaper)?;
    println!("\n## output\n\n{}", std::str::from_utf8(&output).unwrap());

    Ok(())
}

enum ProcessorType {
    LazyLoading,
    HtmlEscape,
}

impl ProcessorType {
    fn build<'write, W>(&self, mut output: W) -> Box<dyn Processor + 'write>
    where
        W: std::io::Write + 'write
    {
        match self {
            ProcessorType::LazyLoading => Box::new(HtmlRewriter::new(
                Settings {
                    element_content_handlers: vec![element!("img", |el| {
                        el.set_attribute("loading", "lazy")?;
                        Ok(())
                    })],
                    ..Default::default()
                },
                move |buffer: &[u8]| output.write_all(buffer).unwrap(),
            )),
            ProcessorType::HtmlEscape => Box::new(Escaper { output }),
        }
    }
}

trait Processor {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>>;
    fn end(self: Box<Self>) -> Result<(), Box<dyn Error>>;
}

// Code will be simplified if input here is &[u8]
// But then printlns will all need to be updated
//
// By default Processor trait has 'static lifetime that was added as amend RFC
// https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md
// This is consistent with the mental model of "once you box up an object,
// you must add annotations for it to contain borrowed data".
fn process(input: &str, mut processor: Box<dyn Processor + '_>) -> Result<(), Box<dyn Error>> {
    processor.write(input.as_bytes())?;
    processor.end()?;
    Ok(())
}

impl<'processor, Output: OutputSink> Processor for HtmlRewriter<'processor, Output> {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>> {
        HtmlRewriter::write(self, chunk).map_err(Into::into)
    }

    fn end(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        HtmlRewriter::end(*self).map_err(Into::into)
    }
}

impl<Write: std::io::Write> Processor for Escaper<Write> {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>> {
        encode_safe_to_writer(std::str::from_utf8(chunk)?, &mut self.output).map_err(Into::into)
    }

    fn end(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct Escaper<Write: std::io::Write> {
    output: Write,
}
