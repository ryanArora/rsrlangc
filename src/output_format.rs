use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum OutputFormat {
    Binary,
}
