use std::error::Error;

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
    //rewriter.write(input.as_bytes())?;
    //rewriter.end()?;

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