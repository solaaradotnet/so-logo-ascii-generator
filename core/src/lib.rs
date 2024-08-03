use chrono::Datelike;
use figrs::{Figlet, FigletOptions};
use std::cmp::max;

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

pub fn generate(msg: &str, with_copyright: bool) -> Result<String, SoLogoAsciiGeneratorError> {
    let opt = FigletOptions {
        font: "Big".to_string(), // Default font is "Standard"
        ..FigletOptions::default()
    };
    let figure = Figlet::text(msg.to_owned(), opt)
        .map_err(SoLogoAsciiGeneratorError::TextGenerationError)?
        .text;

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

    let last_row_copyright_text = if with_copyright {
        let current_time = chrono::Local::now();
        let current_year = current_time.year();
        let copyright_string = format!("(C) Solaara's Network {}", current_year);

        let padding_length = if max_width >= (SOLAARA_LOGO_LAST_ROW.len() + copyright_string.len())
        {
            max_width - SOLAARA_LOGO_LAST_ROW.len() - copyright_string.len()
        } else {
            4
        };

        " ".repeat(padding_length) + &copyright_string
    } else {
        "".to_owned()
    };

    Ok(SOLAARA_LOGO_TOP_HALF.to_owned()
        + &bottom_half_with_text
        + SOLAARA_LOGO_LAST_ROW
        + &last_row_copyright_text)
}

#[derive(thiserror::Error, Debug)]
pub enum SoLogoAsciiGeneratorError {
    #[error("Error generating ascii art text: {0}")]
    TextGenerationError(String),
}
