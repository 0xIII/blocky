use crate::html::html::{HtmlElement, ToHtml};
use chrono::NaiveDate;
use lazy_regex::regex_captures;
use std::{
    convert::From,
    fs,
    io::{BufRead, BufReader, Result},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct StencilHeader {
    route: String,
    date: NaiveDate,
}

impl From<String> for StencilHeader {
    fn from(raw: String) -> Self {
        let raw_header: Vec<&str> = raw
            .lines()
            .filter(|line| !line.is_empty())
            .take(2)
            .collect();
        let date = NaiveDate::parse_from_str(
            &raw_header[1].to_string()[..&raw_header[1].to_string().len() - 1],
            "%d/%m/%Y",
        )
        .unwrap();
        StencilHeader {
            route: raw_header[0].to_string(),
            date: date,
        }
    }
}

#[derive(Debug)]
pub struct Stencil {
    header: StencilHeader,
    content: Vec<HtmlElement>,
}

impl Stencil {
    fn filter_words<'a>(words: impl Iterator<Item = &'a str>) -> Vec<HtmlElement> {
        words
            .map(|word| {
                let regex = regex_captures!(r#"&\[([^\]]*)]\(([^)]*)\)"#, &word);
                match regex {
                    Some((_, attributes, inner)) => HtmlElement::REF(
                        vec![HtmlElement::TEXT(inner.to_string())],
                        attributes.to_string(),
                    ),
                    None => HtmlElement::TEXT(word.to_string()),
                }
            })
            .collect()
    }

    pub fn content_from_string(raw: String) -> Vec<HtmlElement> {
        let lines = raw
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut words = line.split_inclusive(" ").peekable();
                match words.peek() {
                    Some(&"! ") => {
                        HtmlElement::HEADING(Stencil::filter_words(words.skip(1)), String::from(""))
                    }
                    Some(&"- ") => {
                        HtmlElement::LIST(Stencil::filter_words(words.skip(1)), String::from(""))
                    }
                    _ => HtmlElement::PARAGRAPH(Stencil::filter_words(words), String::from("")),
                }
            })
            .collect::<Vec<HtmlElement>>();
        lines
    }

    pub fn first_line(path: PathBuf) -> (String, String) {
        let file = match fs::File::open(&path) {
            Ok(file) => file,
            Err(_) => panic!("Unable to read title from {:?}", &path),
        };
        let mut buffer = BufReader::new(file);
        let mut first_line = String::new();
        let _ = buffer.read_line(&mut first_line);
        let (route, stencil) = first_line.split_once(":").unwrap();

        (String::from(route), String::from(stencil))
    }
}

impl From<String> for Stencil {
    fn from(raw: String) -> Self {
        let post: (&str, &str) = raw.split_once("\n\n").unwrap();
        log::info!("{:?}", post);
        let content: Vec<HtmlElement> = Stencil::content_from_string(post.1.to_string());
        Stencil {
            header: StencilHeader::from(post.0.to_string()),
            content: content,
        }
    }
}

impl ToHtml for Stencil {
    fn to_html(&self) -> String {
        self.content
            .iter()
            .map(|elem| elem.to_html())
            .collect::<String>()
    }
}

const STENCIL_FOLDER: &str = "./www/posts";

pub fn list_available_stencils() -> Result<Vec<fs::DirEntry>> {
    let stencil_list = fs::read_dir(STENCIL_FOLDER)?
        .map(|x| {
            let dir = x.unwrap();
            log::info!("Found stencil {:?}", dir);
            dir
        })
        .collect();
    Ok(stencil_list)
}
