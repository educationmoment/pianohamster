# Piano Hamster


![Logo](hamster-logo.png)



A piano programming language that compiles to MIDI. PianoHamster supports Right Hand, Left Hand ,pedal, chords, rests, bars, tempo/time/key, and compiles to a MIDI file. 


Created entirely in RUST. (why not?)



## output

**format**: standard MIDI file (SMF), **format 1**, PPQ 480

---

## features

- **notes & chords**: `note C4 q`, `chord [C4 E4 G4] h`
- **pedal**: `pedal on/off`, `pedal q` 
- **tempo / time / key**: `tempo 84`, `time 4/4`, `key major 0`
- **instrument**: general insturment, example: `instrument 0` is acoustic grand
- **portable output**: open in any midi player

---

### usage
```text
pianohamster <input.ps> [-o <output.mid>]
```

### example
```text
pianohamster song.ps
pianohamster song.ps -o out/song.mid
```
---
## syntax

```text
Program   := Line*
Line      := Comment | Bar | HeaderStmt | MusicStmt
Comment   := '#' .*
Bar       := '|'

HeaderStmt := Tempo | TimeSig | KeySig | Instrument | Velocity
Tempo     := 'tempo' INT
TimeSig   := 'time' INT '/' INT
KeySig    := 'key' ('major'|'min'|'minor') INT        # sharps/flats (-7..+7)
Instrument:= 'instrument' INT
Velocity  := 'velocity' INT

MusicStmt := Voice | Note | Chord | Rest | Pedal
Voice     := 'voice' ('RH'|'LH')
Note      := 'note' Pitch Dur ['vel' INT]
Chord     := 'chord' '[' Pitch+ ']' Dur
Rest      := 'rest' Dur
Pedal     := 'pedal' ('on'|'off'|Dur)

Pitch     := NOTE_NAME OCTAVE   # e.g., C4, F#3, Bb2
Dur       := ('w'|'h'|'q'|'e'|'s') ['.']
```




### how to compile

For MacOS/Linux:
```text 
cargo build --release

cat > example.ps <<'EOF'
tempo 100
time 4/4
key major 0
instrument 0
velocity 85

voice LH
chord [C3 G3] h
rest q
chord [F2 C3] q

voice RH
note C4 q
note E4 q
note G4 q
note C5 q
EOF

# compile
./target/release/pianohamster example.ps -o example.mid

# open your midi file in whatever
```

For Windows Powershell:

```text
cargo build --release

@"
tempo 100
time 4/4
key major 0
instrument 0
velocity 85

voice LH
chord [C3 G3] h
rest q
chord [F2 C3] q

voice RH
note C4 q
note E4 q
note G4 q
note C5 q
"@ | Out-File -Encoding ASCII example.ps

# compile
.\target\release\pianoscript.exe example.ps -o example.mid

# open your midi file in whatever

```


## faq / common bugs
```text
1) parse error / “unknown command” (line N)

# Check for typos (vel not val, voice RH not voice righthand, etc.).

2) “invalid duration token”
# Only w h q e s with optional dot (.) are recognized, e.g., q. not qd.



Why did I make this?

I wanted to make my own programming language. also i wanted to learn rust :).

hamster :p




