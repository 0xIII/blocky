pub trait ToHtml {
    fn to_html(&self) -> String;
}

#[derive(Debug)]
pub enum HtmlElement {
    HEADING(Vec<HtmlElement>, String),
    LIST(Vec<HtmlElement>, String),
    PARAGRAPH(Vec<HtmlElement>, String),
    REF(Vec<HtmlElement>, String),
    TEXT(String),
}

impl ToHtml for HtmlElement {
    fn to_html(&self) -> String {
        match self {
            HtmlElement::HEADING(inner, attributes) => format!("<h4 {attributes}>{inner}</h4>", inner=inner.iter().map(|elem| elem.to_html()).collect::<String>()),
            HtmlElement::LIST(inner, attributes) => format!("<li {attributes}>{inner}</li>", inner=inner.iter().map(|elem| elem.to_html()).collect::<String>()),
            HtmlElement::PARAGRAPH(inner, attributes) => format!("<p {attributes}>{inner}</p>", inner=inner.iter().map(|elem| elem.to_html()).collect::<String>()),
            HtmlElement::REF(inner, attributes) => format!("<a {attributes}>{inner}</a>", inner=inner.iter().map(|elem| elem.to_html()).collect::<String>()),
            HtmlElement::TEXT(text) => text.to_string(),
        }
    }
}
