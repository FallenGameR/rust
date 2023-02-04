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

enum ProcessorImpl<W: std::io::Write> {
    LazyLoading(HtmlRewriter<'static, WriterOutputSink<W>>) ,
    HtmlEscape(Escaper<W>),
}

struct WriterOutputSink<W> {
    writer: W
}

impl<W: std::io::Write> OutputSink for WriterOutputSink<W> {
    fn handle_chunk(&mut self, chunk: &[u8]) {
        self.writer.write_all(chunk).unwrap()
    }
}

impl ProcessorType {
    fn build<'w, W: std::io::Write + 'w>(&self, output: W) -> ProcessorImpl<W> {
        match self {
            ProcessorType::LazyLoading => ProcessorImpl::LazyLoading(HtmlRewriter::new(
                Settings {
                    element_content_handlers: vec![element!("img", |el| {
                        el.set_attribute("loading", "lazy")?;
                        Ok(())
                    })],
                    ..Default::default()
                },
                WriterOutputSink { writer: output },
            )),
            ProcessorType::HtmlEscape => ProcessorImpl::HtmlEscape(Escaper { output }),
        }
    }
}

impl<W: std::io::Write> Processor for ProcessorImpl<W> {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>> {
        match self {
            ProcessorImpl::LazyLoading(processor) => processor.write(chunk).map_err(Into::into),
            ProcessorImpl::HtmlEscape(processor) => encode_safe_to_writer(std::str::from_utf8(chunk)?, &mut processor.output).map_err(Into::into),
        }
    }

    fn end(self) -> Result<(), Box<dyn Error>> {
        match self {
            ProcessorImpl::LazyLoading(processor) => processor.end().map_err(Into::into),
            ProcessorImpl::HtmlEscape(_) => Ok(()),
        }
    }
}

trait Processor {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>>;
    fn end(self) -> Result<(), Box<dyn Error>>;
}

// Code will be simplified if input here is &[u8]
// But then printlns will all need to be updated
//
// By default Processor trait has 'static lifetime that was added as amend RFC
// https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md
// This is consistent with the mental model of "once you box up an object,
// you must add annotations for it to contain borrowed data".
fn process<P: Processor>(input: &str, mut processor: P) -> Result<(), Box<dyn Error>> {
    processor.write(input.as_bytes())?;
    processor.end()?;
    Ok(())
}

struct Escaper<Write: std::io::Write> {
    output: Write,
}
