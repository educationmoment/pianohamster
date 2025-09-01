use thiserror::Error;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurBase { W, H, Q, E, S }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dur { pub base: DurBase, pub dotted: bool }


#[derive(Debug, Error)]
pub enum DurParseError { #[error("invalid duration token: {0}")] Invalid(String) }


impl Dur {
pub const PPQ: u32 = 480; // our pulses per quarter


pub fn parse(tok: &str) -> Result<Self, DurParseError> {
let dotted = tok.ends_with('.') ;
let base_str = if dotted { &tok[..tok.len()-1] } else { tok };
let base = match base_str {
"w" => DurBase::W,
"h" => DurBase::H,
"q" => DurBase::Q,
"e" => DurBase::E,
"s" => DurBase::S,
_ => return Err(DurParseError::Invalid(tok.to_string())),
};
Ok(Dur { base, dotted })
}


pub fn ticks(&self) -> u32 {
let base = match self.base {
DurBase::W => Self::PPQ * 4,
DurBase::H => Self::PPQ * 2,
DurBase::Q => Self::PPQ,
DurBase::E => Self::PPQ / 2,
DurBase::S => Self::PPQ / 4,
};
if self.dotted { base + base / 2 } else { base }
}
}