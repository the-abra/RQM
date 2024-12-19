use ratatui::{widgets::Paragraph, text::Span};
use rust_i18n::t;
pub fn t(key: &str) -> String {
    t!(key).to_string()
}

pub fn render() -> Paragraph<'static> {
    Paragraph::new(Span::raw(t("Network Settings Content")))
}