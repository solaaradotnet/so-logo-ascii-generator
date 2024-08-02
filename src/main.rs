use std::{cmp::max, collections::btree_set::Difference};

use chrono::Datelike;
use clap::Parser;
use figrs::{Figlet, FigletOptions};

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

#[derive(Debug, Parser)]
struct CliArgs {
    pub text: String,

    #[arg(short = 'c', long, default_value = "false")]
    pub with_copyright: bool,
}

fn main() {
    let args = CliArgs::parse();

    let opt = FigletOptions {
        font: "Big".to_string(), // Default font is "Standard"
        ..FigletOptions::default()
    };
    let figure = Figlet::text(args.text.to_owned(), opt).unwrap().text;

    let mut figure_lines = figure.lines();
    let (new_bottom_half, max_width) = SOLAARA_LOGO_BOTTOM_HALF
        .lines()
        .map(|logo_row| logo_row.to_owned() + (figure_lines.next().unwrap_or("")))
        .fold((String::new(), 0), |mut acc, el| {
            acc.0.push_str(&el);
            acc.0.push('\n');
            acc.1 = max(acc.1, el.len());
            acc
        });

    let last_row = if args.with_copyright {
        let current_time = chrono::Local::now();
        let current_year = current_time.year();
        let copyright_string = format!("(C) Solaara's Network {}", current_year);

        SOLAARA_LOGO_LAST_ROW.to_owned()
            + &" ".repeat(
                if max_width >= (SOLAARA_LOGO_LAST_ROW.len() + copyright_string.len()) {
                    max_width - SOLAARA_LOGO_LAST_ROW.len() - copyright_string.len()
                } else {
                    4
                },
            )
            + &copyright_string
    } else {
        SOLAARA_LOGO_LAST_ROW.to_owned()
    };

    let finished_thing = SOLAARA_LOGO_TOP_HALF.to_owned() + &new_bottom_half + &last_row;
    println!("{finished_thing}")
}
