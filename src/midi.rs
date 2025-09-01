use crate::duration::Dur;


fn vlq(mut n: u32) -> Vec<u8> {
let mut bytes = vec![(n & 0x7F) as u8];
n >>= 7;
while n > 0 { bytes.insert(0, ((n & 0x7F) as u8) | 0x80); n >>= 7; }
bytes
}


fn meta(delta: u32, ty: u8, data: &[u8]) -> Vec<u8> {
let mut v = vlq(delta); v.extend_from_slice(&[0xFF, ty, data.len() as u8]); v.extend_from_slice(data); v
}


fn note_on(delta: u32, ch: u8, note: u8, vel: u8) -> Vec<u8> {
let mut v = vlq(delta); v.extend_from_slice(&[0x90 | (ch & 0x0F), note, vel]); v
}
fn note_off(delta: u32, ch: u8, note: u8, vel: u8) -> Vec<u8> {
let mut v = vlq(delta); v.extend_from_slice(&[0x80 | (ch & 0x0F), note, vel]); v
}
fn program_change(delta: u32, ch: u8, prog: u8) -> Vec<u8> {
let mut v = vlq(delta); v.extend_from_slice(&[0xC0 | (ch & 0x0F), prog]); v
}
fn control_change(delta: u32, ch: u8, cc: u8, val: u8) -> Vec<u8> {
let mut v = vlq(delta); v.extend_from_slice(&[0xB0 | (ch & 0x0F), cc, val]); v
}
fn text_event(delta: u32, ty: u8, s: &str) -> Vec<u8> { // ty: 0x03=TrackName
let b = s.as_bytes();
let mut v = vlq(delta); v.extend_from_slice(&[0xFF, ty, b.len() as u8]); v.extend_from_slice(b); v
}


fn track_chunk(events: &[u8]) -> Vec<u8> {
let mut v = b"MTrk".to_vec();
v.extend_from_slice(&(events.len() as u32).to_be_bytes());
v.extend_from_slice(events);
v
}


fn header_chunk(fmt: u16, ntrks: u16, division: u16) -> Vec<u8> {
let mut v = b"MThd".to_vec();
v.extend_from_slice(&6u32.to_be_bytes());
v.extend_from_slice(&fmt.to_be_bytes());
v.extend_from_slice(&ntrks.to_be_bytes());
v.extend_from_slice(&division.to_be_bytes());
v
}


pub struct MidiBuilder {
pub ppq: u16,
pub conductor: Vec<u8>,
pub rh: Vec<u8>,
pub lh: Vec<u8>,
}


impl MidiBuilder {
pub fn new(ppq: u16) -> Self { Self { ppq, conductor: Vec::new(), rh: Vec::new(), lh: Vec::new() } }


pub fn build(mut self) -> Vec<u8> {
// End of track
self.conductor.extend_from_slice(&meta(0, 0x2F, &[]));
self.rh.extend_from_slice(&meta(0, 0x2F, &[]));
self.lh.extend_from_slice(&meta(0, 0x2F, &[]));


let mut out = header_chunk(1, 3, self.ppq);
out.extend_from_slice(&track_chunk(&self.conductor));
out.extend_from_slice(&track_chunk(&self.rh));
out.extend_from_slice(&track_chunk(&self.lh));
out
}


pub fn set_names(&mut self) {
self.conductor.extend_from_slice(&text_event(0, 0x03, "Conductor"));
self.rh.extend_from_slice(&text_event(0, 0x03, "Right Hand"));
self.lh.extend_from_slice(&text_event(0, 0x03, "Left Hand"));
}


pub fn tempo(&mut self, bpm: u32) {
let us_per_q = (60_000_000u32 / bpm.max(1)) as u32;
let data = [(us_per_q >> 16) as u8, (us_per_q >> 8) as u8, (us_per_q & 0xFF) as u8];
self.conductor.extend_from_slice(&meta(0, 0x51, &data));
}


pub fn time_sig(&mut self, num: u8, den: u8) {
let mut pow = 0; let mut d = 1u8; while d < den && pow < 7 { d <<= 1; pow += 1; }
self.conductor.extend_from_slice(&meta(0, 0x58, &[num, pow, 24, 8]));
}