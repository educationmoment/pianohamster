use crate::duration::Dur;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceSel { RH, LH }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode { Major, Minor }


#[derive(Debug, Clone)]
pub struct Header {
pub tempo_bpm: Option<u32>,
pub time_num: Option<u8>,
pub time_den: Option<u8>,
pub key_mode: Option<Mode>,
pub key_sf: Option<i8>, // to definesharps& flats -7..+7
pub velocity: Option<u8>,
pub instrument: Option<u8>, // GM program 0..127
}


impl Default for Header {
fn default() -> Self {
Self { tempo_bpm: None, time_num: None, time_den: None, key_mode: None, key_sf: None, velocity: None, instrument: None }
}
}


#[derive(Debug, Clone)]
pub enum Stmt {
Bar,
Voice(VoiceSel),
Velocity(u8),
Note { pitch: String, dur: Dur, vel: Option<u8> },
Rest { dur: Dur },
Chord { pitches: Vec<String>, dur: Dur },
PedalOn,
PedalOff,
PedalFor(Dur),
}


#[derive(Debug, Clone)]
pub struct Program {
pub header: Header,
pub stmts: Vec<Stmt>,
}