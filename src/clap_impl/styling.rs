use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Style};

pub fn styles() -> Styles { 
    Styles::plain()
        .placeholder(placeholder_style())
        .header(header_style())
        .usage(usage_style())
        .error(error_style())
        .context(context_style())
}

fn placeholder_style() -> Style { Style::new().dimmed() }
fn header_style() -> Style { AnsiColor::Yellow.on_default() }
fn usage_style() -> Style { AnsiColor::Yellow.on_default() }
fn error_style() -> Style { AnsiColor::Red.on_default() }
fn context_style() -> Style { AnsiColor::Yellow.on_default().dimmed() }