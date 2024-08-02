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
              .@@.
              :@% 
";

#[derive(Debug, Parser)]
struct CliArgs {
    pub text: String,
}

fn main() {
    let args = CliArgs::parse();

    let opt = FigletOptions {
        font: "Big".to_string(), // Default font is "Standard"
        ..FigletOptions::default()
    };
    let figure = Figlet::text(args.text.to_owned(), opt).unwrap().text;

    let mut figure_lines = figure.lines();
    let new_bottom_half: Vec<String> = SOLAARA_LOGO_BOTTOM_HALF
        .lines()
        .map(|logo_row| logo_row.to_owned() + (figure_lines.next().unwrap_or("")))
        .collect();
    let new_bottom_half = new_bottom_half.join("\n");

    let finished_thing = SOLAARA_LOGO_TOP_HALF.to_owned() + &new_bottom_half;

    println!("{finished_thing}")
}
