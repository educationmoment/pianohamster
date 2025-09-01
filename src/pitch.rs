use thiserror::Error;


#[derive(Debug, Error)]
pub enum PitchError { #[error("invalid pitch: {0}")] Invalid(String) }


fn note_index(name: &str) -> Option<i32> {
Some(match name {
"C" => 0, "C#"|"Db" => 1, "D" => 2, "D#"|"Eb" => 3, "E" => 4,
"F" => 5, "F#"|"Gb" => 6, "G" => 7, "G#"|"Ab" => 8, "A" => 9,
"A#"|"Bb" => 10, "B" => 11, _ => return None,
})
}


/// Returns MIDI note number (e.g., C4 -> 60)
pub fn to_midi(pitch: &str) -> Result<u8, PitchError> {
if pitch.len() < 2 { return Err(PitchError::Invalid(pitch.into())); }
let (name, oct_str) = pitch.split_at(pitch.find(|c: char| c.is_ascii_digit()).ok_or_else(|| PitchError::Invalid(pitch.into()))?);
let idx = note_index(name).ok_or_else(|| PitchError::Invalid(pitch.into()))?;
let oct: i32 = oct_str.parse().map_err(|_| PitchError::Invalid(pitch.into()))?;
let midi = (oct + 1) * 12 + idx;
if midi < 0 || midi > 127 { return Err(PitchError::Invalid(pitch.into())); }
Ok(midi as u8)
}