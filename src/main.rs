mod arch;
mod ir;
mod output_format;

use arch::Arch;
use output_format::OutputFormat;

#[derive(clap::Parser, Debug)]
struct Args {
    input_file: std::path::PathBuf,
    #[arg(short, default_value = "a.bin")]
    output_file: std::path::PathBuf,
    #[arg(long, value_enum, default_value_t = Arch::X86_64)]
    arch: Arch,
    #[arg(long, value_enum, default_value_t = OutputFormat::Binary)]
    output_format: OutputFormat,
}

fn main() {
    let _args = <Args as clap::Parser>::parse();
}
