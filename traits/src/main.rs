use std::error::Error;

use enum_dispatch::enum_dispatch;
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

impl<'processor, Output: OutputSink> Processor for HtmlRewriter<'processor, Output> {
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

#[enum_dispatch]
trait Processor {
    fn write(&mut self, chunk: &[u8]) -> Result<(), Box<dyn Error>>;
    fn end(self) -> Result<(), Box<dyn Error>>;
}

#[enum_dispatch(Processor)]
enum ProcessorImpl<W: std::io::Write> {
    LazyLoading(HtmlRewriter<'static, WriterOutputSink<W>>) ,
    HtmlEscape(Escaper<W>),
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
