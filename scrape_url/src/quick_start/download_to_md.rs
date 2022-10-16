use std::fs;

// 下载网页文件并转成md文件
pub fn download_file_to_markdown() -> Result<(), Box<dyn, std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body: String = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md: String = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
