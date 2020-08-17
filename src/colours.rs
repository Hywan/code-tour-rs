#[cfg(feature = "colours")]
use ansi_term::{Colour, Style};

pub fn comment(comment: String) -> String {
    #[cfg(feature = "colours")]
    {
        Style::new()
            .fg(Colour::Fixed(253))
            .on(Colour::Fixed(238))
            .paint(comment)
            .to_string()
    }

    #[cfg(not(feature = "colours"))]
    {
        comment
    }
}

pub fn statement(statement: String) -> String {
    #[cfg(feature = "colours")]
    {
        Style::new()
            .fg(Colour::Fixed(228))
            .paint(format!("▶︎    {}", statement))
            .to_string()
    }

    #[cfg(not(feature = "colours"))]
    {
        statement
    }
}
