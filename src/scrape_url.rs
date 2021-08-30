use std::fs;

pub fn scrape(url: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {

    println!("Fetuching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}", output);

    Ok(())
}