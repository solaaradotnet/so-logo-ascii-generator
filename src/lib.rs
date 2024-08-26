use chrono::Datelike;
use figrs::{Figlet, FigletOptions};
use std::cmp::max;

/// solaara logo split up in convenient sections
const SOLAARA_LOGO_TOP_HALF: &str = "                 =@-
     =%.         *@:          .
     .%@=        %@.        .=@#
       =@%.    ..-=.      .*@@:
        .#=.-@@%##%@@=. .#@%:
         .*@=.       =@#.=.
        .@*            +@:
==--:.. *#.             ##
++*#%@* @=              .@.
";
const SOLAARA_LOGO_BOTTOM_HALF: &str = "        +@.       
         +@-      
      .+%::#@+.   
    .*@@-   .:+%@.
   .*%:        ##.
              .@@.";
const SOLAARA_LOGO_LAST_ROW: &str = "              :@% ";

/// main generate function
pub fn generate(
    msg: &str,
    with_copyright: bool,
    font: TextFont,
) -> Result<String, SoLogoAsciiGeneratorError> {
    let opt = FigletOptions {
        font: font.to_string(),
        ..FigletOptions::default()
    };

    // create a figlet-like text
    let figure = Figlet::text(msg.to_owned(), opt)
        .map_err(SoLogoAsciiGeneratorError::TextGenerationError)?
        .text;

    // fix if font is
    let figure = match font {
        TextFont::Shadow => "\n".to_string() + &figure,
        _ => figure,
    };

    // stitch the bottom half of the logo with the figlet and get the max length of the whole thing
    // aswell
    let (bottom_half_with_text, max_width) = SOLAARA_LOGO_BOTTOM_HALF
        .lines()
        .zip(figure.lines())
        .map(|(logo_row, text_row)| logo_row.to_owned() + text_row)
        .fold((String::new(), 0), |mut acc, el| {
            acc.0.push_str(&el);
            acc.0.push('\n');
            acc.1 = max(acc.1, el.len());
            acc
        });

    // if we want the copyright text, generate it and stitch it together with the last row
    let last_row_copyright_text = if with_copyright {
        let current_time = chrono::Local::now();
        let current_year = current_time.year();
        let copyright_string = format!("(C) Solaara's Network {}", current_year);

        let padding_length = if max_width >= (SOLAARA_LOGO_LAST_ROW.len() + copyright_string.len())
        // if the total width of the logo text is large enough for the copyright string to fit
        {
            // pad it so the copyright text is right aligned and flush with the logo text
            max_width - SOLAARA_LOGO_LAST_ROW.len() - copyright_string.len()
        } else {
            // else pad it with 4 spaces (quite arbitrary)
            4
        };

        " ".repeat(padding_length) + &copyright_string
    } else {
        "".to_owned()
    };

    // assemble the whole thing
    Ok(SOLAARA_LOGO_TOP_HALF.to_owned()
        + &bottom_half_with_text
        + SOLAARA_LOGO_LAST_ROW
        + &last_row_copyright_text)
}

#[derive(Copy, Clone, Default, Debug)]
pub enum TextFont {
    #[default]
    Big,
    Shadow,
}

impl std::fmt::Display for TextFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextFont::Big => "Big",
                TextFont::Shadow => "Shadow",
            }
        )
    }
}

/// main error handling enum
#[derive(thiserror::Error, Debug)]
pub enum SoLogoAsciiGeneratorError {
    #[error("Error generating ascii art text: {0}")]
    TextGenerationError(String),

    #[error("Invalid font specified: {0}")]
    InvalidFont(String),
}
