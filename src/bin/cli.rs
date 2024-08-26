use so_logo_ascii_generator::generate;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    pub text: String,

    #[arg(short = 'c', long, default_value = "false")]
    pub with_copyright: bool,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let out = generate(&args.text, args.with_copyright)?;

    println!("{out}");
    Ok(())
}
