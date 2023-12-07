use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Arch {
    X86_64,
}
