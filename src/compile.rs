use anyhow::Result;
use crate::ast::{Program, Stmt, VoiceSel, Mode};
use crate::duration::Dur;
use crate::pitch::to_midi;
use crate::midi::MidiBuilder;


pub fn compile_to_midi(prog: Program) -> Result<Vec<u8>> {
let mut mb = MidiBuilder::new(Dur::PPQ as u16);
mb.set_names();


// Header/meta defaults
mb.tempo(prog.header.tempo_bpm.unwrap_or(90));
mb.time_sig(prog.header.time_num.unwrap_or(4), prog.header.time_den.unwrap_or(4));
mb.key_sig(prog.header.key_sf.unwrap_or(0), matches!(prog.header.key_mode, Some(Mode::Minor)));
mb.program(0, prog.header.instrument.unwrap_or(0));


let mut cur_voice = VoiceSel::RH;
let mut cur_vel: u8 = prog.header.velocity.unwrap_or(80);


for stmt in prog.stmts {
match stmt {
Stmt::Bar => { /* purely cosmetic here */ },
Stmt::Voice(v) => cur_voice = v,
Stmt::Velocity(v) => cur_vel = v.min(127),
Stmt::Note { pitch, dur, vel } => {
let note = to_midi(&pitch)?;
let right = matches!(cur_voice, VoiceSel::RH);
mb.note(right, note, dur.ticks(), vel.unwrap_or(cur_vel));
}
Stmt::Rest { dur } => {
let right = matches!(cur_voice, VoiceSel::RH);
mb.rest(right, dur.ticks());
}
Stmt::Chord { pitches, dur } => {
let right = matches!(cur_voice, VoiceSel::RH);
let mut notes = Vec::new();
for p in pitches { notes.push(to_midi(&p)?); }
mb.chord(right, &notes, dur.ticks(), cur_vel);
}
Stmt::PedalOn => {
let right = matches!(cur_voice, VoiceSel::RH);
mb.pedal(right, true, 0);
}
Stmt::PedalOff => {
let right = matches!(cur_voice, VoiceSel::RH);
mb.pedal(right, false, 0);
}
Stmt::PedalFor(dur) => {
let right = matches!(cur_voice, VoiceSel::RH);
mb.pedal(right, true, 0);
mb.pedal(right, false, dur.ticks());
}
}
}


Ok(mb.build())
}