use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::{Context, Result};


mod ast; mod lexer; mod parser; mod duration; mod pitch; mod midi; mod compile;


#[derive(Parser)]
#[command(name="pianoscript", version, about="PianoScript -> MIDI compiler")]
struct Cli {
/// Input .ps file
input: PathBuf,


/// Output .mid file (defaults to input basename + .mid)
#[arg(short, long)]
out: Option<PathBuf>,
}


fn main() -> Result<()> {
let cli = Cli::parse();
let src = fs::read_to_string(&cli.input).with_context(|| format!("reading {}", cli.input.display()))?;
let prog = parser::parse(&src).context("parse error")?;
let midi = compile::compile_to_midi(prog).context("compile error")?;
let out = cli.out.unwrap_or_else(|| cli.input.with_extension("mid"));
fs::write(&out, midi).with_context(|| format!("writing {}", out.display()))?;
println!("Wrote {}", out.display());
Ok(())
}