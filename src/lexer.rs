#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
Word(String),
LBrack, RBrack,
Bar, // '|'
Comment, // whole-line comment, already skipped
}


/// Simple whitespace/token splitter suitable for our line-based language.
pub fn lex_line(line: &str) -> Vec<Tok> {
let s = line.trim();
if s.is_empty() || s.starts_with('#') { return vec![Tok::Comment]; }
if s == "|" { return vec![Tok::Bar]; }


let mut out = Vec::new();
let mut cur = String::new();
for ch in s.chars() {
match ch {
'[' => { if !cur.is_empty() { out.push(Tok::Word(cur.clone())); cur.clear(); } out.push(Tok::LBrack); },
']' => { if !cur.is_empty() { out.push(Tok::Word(cur.clone())); cur.clear(); } out.push(Tok::RBrack); },
' ' | '\t' => { if !cur.is_empty() { out.push(Tok::Word(cur.clone())); cur.clear(); } },
_ => cur.push(ch),
}
}
if !cur.is_empty() { out.push(Tok::Word(cur)); }
out
}
