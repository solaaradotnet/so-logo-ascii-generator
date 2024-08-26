use so_logo_ascii_generator::generate;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    pub text: String,

    #[arg(short = 'c', long, default_value = "false")]
    pub with_copyright: bool,

    #[arg(short = 'f', long, default_value = "big")]
    pub font: CliTextFont,
}

#[derive(Default, ValueEnum, Debug, Copy, Clone)]
enum CliTextFont {
    #[default]
    Big,
    Shadow,
}

impl From<CliTextFont> for so_logo_ascii_generator::TextFont {
    fn from(value: CliTextFont) -> Self {
        match value {
            CliTextFont::Big => Self::Big,
            CliTextFont::Shadow => Self::Shadow,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let out = generate(&args.text, args.with_copyright, args.font.into())?;

    println!("{out}");
    Ok(())
}
