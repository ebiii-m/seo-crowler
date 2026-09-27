use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;

pub fn parse_sitemap(sitemap_path: &str) -> Vec<String> {
    let content = fs::read(sitemap_path).unwrap_or_default();
    let mut reader = Reader::from_reader(&content[..]);
    reader.trim_text(true);
    
    let mut urls = Vec::new();
    
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"loc" {
                    let mut url = String::new();
                    if let Ok(text) = reader.read_text(e.name()) {
                        url = text.into_owned().trim().to_string();
                    }
                    urls.push(url);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Failed to parse sitemap: {}", e),
            _ => {}
        }
    }
    
    urls
}
