use cached::proc_macro::cached;
use eframe::egui::text::LayoutJob;
use eframe::egui::{Color32, FontId, TextFormat};
use egui::FontFamily;
use lazy_static::lazy_static;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use tree_sitter_html;
use tree_sitter_javascript;
use tree_sitter_typescript;

lazy_static! {
    static ref AST_TOKEN_TYPES: [(&'static str, &'static str); 20] = [
        ("attribute", "#9cdcfe"),
        ("constant", "#569cd6"),
        ("function.builtin", "#C8C8C8"),
        ("function", "#C8C8C8"),
        ("keyword", "#569CD6"),
        ("operator", "#b4b4b4"),
        ("property", "#DADADA"),
        ("punctuation", "#b4b4b4"),
        ("punctuation.bracket", "#b4b4b4"),
        ("punctuation.delimiter", "#b4b4b4"),
        ("string", "#ce9178"),
        ("string.special", "#d16969"),
        ("tag", "#569cd6"),
        ("type", "#4EC9B0"),
        ("type.builtin", "#4EC9B0"),
        ("variable", "#C8C8C8"),
        ("variable.builtin", "#C8C8C8"),
        ("variable.parameter", "#7F7F7F"),
        ("variable.builtin", "#C8C8C8"),
        ("comment", "#6A9955"),
    ];
    static ref TOKEN_NAMES: [&'static str; 20] = AST_TOKEN_TYPES.map(|p| p.0);
    static ref HTML_CONFIG: HighlightConfiguration = {
        let mut html_config = HighlightConfiguration::new(
            tree_sitter_html::language(),
            "html",
            tree_sitter_html::HIGHLIGHTS_QUERY,
            tree_sitter_html::INJECTIONS_QUERY,
            "",
        )
        .unwrap();

        html_config.configure(TOKEN_NAMES.as_slice());
        html_config
    };
    static ref JAVASCRIPT_CONFIG: HighlightConfiguration = {
        let mut js_config = HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            "javascript",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTIONS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        )
        .unwrap();

        js_config.configure(TOKEN_NAMES.as_slice());
        js_config
    };
    static ref TYPESCRIPT_CONFIG: HighlightConfiguration = {
        let mut ts_config = HighlightConfiguration::new(
            tree_sitter_typescript::language_typescript(),
            "typescript",
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_javascript::LOCALS_QUERY,
        )
        .unwrap();

        ts_config.configure(TOKEN_NAMES.as_slice());
        ts_config
    };
}

#[cached]
pub fn highlight(code: String) -> LayoutJob {
    let mut highlighter = Highlighter::new();
    let mut job: LayoutJob = LayoutJob::default();
    let bytes = code.as_bytes();

    let events = highlighter
        .highlight(&JAVASCRIPT_CONFIG, &bytes, None, |_| None)
        .unwrap();

    let mut color: &str = "#6A9955";

    for event in events {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                let text = String::from_utf8(bytes.to_vec()).expect("Found invalid UTF-8");

                job.append(
                    &text[start..end],
                    0.0,
                    TextFormat {
                        color: Color32::from_hex(&color).unwrap(),
                        font_id: FontId {
                            family: FontFamily::Monospace,
                            ..FontId::default()
                        },
                        ..TextFormat::default()
                    },
                )
            }
            HighlightEvent::HighlightStart(s) => color = &AST_TOKEN_TYPES[s.0].1,
            HighlightEvent::HighlightEnd => color = "#6A9955",
        }
    }

    job
}
