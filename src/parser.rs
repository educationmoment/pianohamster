use anyhow::{bail, Result};
use crate::ast::{Program, Header, Stmt, VoiceSel, Mode};
use crate::duration::Dur;
use crate::lexer::{lex_line, Tok};

pub fn parse(src: &str) -> Result<Program> {
    let mut header = Header::default();
    let mut stmts = Vec::new();

    for (lineno, raw) in src.lines().enumerate() {
        let toks = lex_line(raw);
        if toks.is_empty() { continue; }
        if toks[0] == Tok::Comment { continue; }
        if toks[0] == Tok::Bar { stmts.push(Stmt::Bar); continue; }

        let words: Vec<String> = toks.iter().filter_map(|t| {
            if let Tok::Word(w) = t { Some(w.clone()) } else { None }
        }).collect();

        let cmd = match words.first() { Some(s) => s.as_str(), None => "" };

        match cmd {
            "tempo" => {
                if words.len() < 2 { bail!("line {}: tempo <bpm>", lineno+1); }
                header.tempo_bpm = Some(words[1].parse()?);
            }
            "time" => {
                if words.len() < 2 { bail!("line {}: time <num>/<den>", lineno+1); }
                let parts: Vec<_> = words[1].split('/').collect();
                if parts.len() != 2 { bail!("line {}: bad time signature", lineno+1); }
                header.time_num = Some(parts[0].parse()?);
                header.time_den = Some(parts[1].parse()?);
            }
            "key" => {
                if words.len() < 3 { bail!("line {}: key <major|min> <sharps/flats>", lineno+1); }
                header.key_mode = Some(match words[1].to_lowercase().as_str() {
                    "minor" | "min" => Mode::Minor,
                    _ => Mode::Major
                });
                header.key_sf = Some(words[2].parse()?);
            }
            "instrument" => {
                if words.len() < 2 { bail!("line {}: instrument <program#>", lineno+1); }
                header.instrument = Some(words[1].parse()?);
            }
            "velocity" => {
                if words.len() < 2 { bail!("line {}: velocity <0..127>", lineno+1); }
                stmts.push(Stmt::Velocity(words[1].parse()?));
            }
            "voice" => {
                if words.len() < 2 { bail!("line {}: voice RH|LH", lineno+1); }
                let which = match words[1].to_uppercase().as_str() {
                    "RH" => VoiceSel::RH,
                    _ => VoiceSel::LH
                };
                stmts.push(Stmt::Voice(which));
            }
            "note" => {
                if words.len() < 3 { bail!("line {}: note <Pitch> <Dur> [vel N]", lineno+1); }
                let pitch = words[1].clone();
                let dur = Dur::parse(&words[2])?;
                let vel = if words.len() >= 5 && words[3].to_lowercase() == "vel" {
                    Some(words[4].parse()?)
                } else { None };
                stmts.push(Stmt::Note { pitch, dur, vel });
            }
            "rest" => {
                if words.len() < 2 { bail!("line {}: rest <Dur>", lineno+1); }
                stmts.push(Stmt::Rest { dur: Dur::parse(&words[1])? });
            }
            "chord" => {
                let toks = lex_line(raw); // need bracket awareness
                let mut pitches = Vec::new();
                let mut inside = false;
                let mut dur_tok: Option<String> = None;
                for t in toks {
                    match t {
                        Tok::LBrack => inside = true,
                        Tok::RBrack => inside = false,
                        Tok::Word(w) => {
                            if w == "chord" { continue; }
                            if inside { pitches.push(w); } else { dur_tok = Some(w); }
                        }
                        _ => {}
                    }
                }
                let d = dur_tok.ok_or_else(|| anyhow::anyhow!("line {}: chord missing duration", lineno+1))?;
                let dur = Dur::parse(&d)?;
                stmts.push(Stmt::Chord { pitches, dur });
            }
            "pedal" => {
                if words.len() == 2 {
                    match words[1].to_lowercase().as_str() {
                        "on" | "down" => stmts.push(Stmt::PedalOn),
                        "off" | "up"  => stmts.push(Stmt::PedalOff),
                        x => stmts.push(Stmt::PedalFor(Dur::parse(x)?)),
                    }
                } else {
                    bail!("line {}: pedal on|off|<Dur>", lineno+1);
                }
            }
            "" => { /* nothing */ }
            other => bail!("line {}: unknown command '{}'", lineno+1, other),
        }
    }

    Ok(Program { header, stmts })
}
