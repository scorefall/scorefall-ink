// ScoreFall Studio - Music Composition Software
//
// Copyright (C) 2019-2020 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
// Copyright (C) 2019 Doug P. Lau
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use muon_rs as muon;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryInto;
use std::str::FromStr;

use cala;

mod fraction;
pub mod note;

pub use fraction::{Fraction, IsZero};
pub use note::{
    Articulation, Note, Pitch, PitchAccidental, PitchClass, PitchName,
    PitchOctave, Steps,
};

/// Cursor pointing to a marking
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Cursor {
    /// Movement number at cursor
    movement: u16,
    /// Measure number at cursor
    measure: u16,
    /// Channel number at curosr
    chan: u16,
    /// Marking number within measure
    marking: u16,
}

impl Cursor {
    /// Create a new cursor
    pub fn new(movement: u16, measure: u16, chan: u16, marking: u16) -> Self {
        Cursor {
            movement,
            measure,
            chan,
            marking,
        }
    }

    /// Create a cursor from the first marking
    pub fn first_marking(&self) -> Self {
        Cursor {
            movement: self.movement,
            measure: self.measure,
            chan: self.chan,
            marking: 0,
        }
    }

    /// Move cursor left.
    pub fn left(&mut self, scof: &Scof) {
        if self.marking > 0 {
            self.marking -= 1;
        } else if self.measure != 0 {
            self.measure -= 1;
            let len = scof.marking_len(self);
            self.marking = if len > 0 { len - 1 } else { 0 };
        }
    }

    /// Move cursor right, and to the next measure if the measure ended.
    pub fn right(&mut self, scof: &Scof) {
        if self.right_checked(scof) {
            // Measure has ended.
            self.measure += 1;
            self.marking = 0;
        }
    }

    /// Fix the cursor if it is wrong.  Move cursor right, and to the next
    /// measure if the measure ended.  FIXME: Maybe remove other API in favor of
    /// this function in conjunction with others.
    pub fn right_fix(&mut self, scof: &Scof) -> bool {
        if self.marking >= scof.marking_len(self) {
            // Measure has ended.
            self.measure += 1;
            self.marking = 0;
            true
        } else {
            false
        }
    }

    /// Move cursor right within the measure, returning true if the measure
    /// ended.  If the measure has ended, the cursor is not changed.
    pub fn right_checked(&mut self, scof: &Scof) -> bool {
        let len = scof.marking_len(self);
        cala::note!("Rgiht CHecka {} {}", self.marking, len);
        if self.marking + 1 < len {
            self.marking += 1;
            false
        } else {
            true
        }
    }

    /// Move cursor to the right within the measure, not checking if it ended.
    pub fn right_unchecked(&mut self) -> Self {
        self.marking += 1;
        self.clone()
    }

    /// Returns true if it's the first bar of music.
    pub fn is_first_bar(&self) -> bool {
        self.measure == 0
    }
}

/// A Dynamic.
#[derive(Clone, Debug, PartialEq)]
pub enum Dynamic {
    PPPPPP,
    PPPPP,
    PPPP,
    PPP,
    PP,
    P,
    MP,
    MF,
    F,
    FF,
    FFF,
    FFFF,
    FFFFF,
    FFFFFF,
    N,
    SF,
    SFZ,
    FP,
    SFP,
}

/// A marking.
#[derive(Clone, Debug, PartialEq)]
pub enum Marking {
    /// Change intensity of sound.
    Dynamic(Dynamic),
    /// Grace Note into
    GraceInto(Note),
    /// Grace Note from
    GraceOutOf(Note),
    /// Note
    Note(Note),
    /// Breath
    Breath,
    /// Short grand pause for all instruments
    CaesuraShort,
    /// Long grand pause for all instruments
    CaesuraLong,
    /// Increase intensity
    Cresc,
    /// Decrease intensity
    Dim,
    /// Pizzicato (pluck)
    Pizz,
    /// Arco (bowed)
    Arco,
    /// Standard Mute [con sordino]
    Mute,
    /// Open (no mute) [senza sordino]
    Open,
    /// Repeat
    Repeat,
}

impl FromStr for Marking {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Marking::Note(s.parse::<Note>().and_then(|a| Ok(a))?))
    }
}

/// A repeat marking for a measure.
pub enum Repeat {
    /// Repeat sign open ||:
    Open,
    /// Repeat sign close :||
    Close,
    /// Sign (to jump backwards to).
    Segno,
    /// Jump back to beginning.
    DC,
    /// Jump back to sign.
    DS,
    /// The marks the beginning of the coda.
    Coda,
    /// Jump forward to the coda.
    ToCoda,
    /// End here (after jumping backwards to the sign).
    Fine,
    /// Numbered ending.
    Ending(u8),
}

/////////////////////
////             ////
/////////////////////

/// A waveform.
#[allow(unused)] // FIXME: Have ability to use waveform
pub struct Waveform {
    /// True: Signed 16-bit integer, False: Signed 8-bit integer.
    si16: bool,
    /// True: Waveform doesn't loop, False: Waveform loops.
    once: bool,
    /// Hexadecimal string representation of waveform.
    wave: String,
}

/// Reverb & other effect settings.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    // TODO
}

/// Channel definition for synthesis.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SynthChan {
    /// Name of instrument sounds.
    waveform: Vec<String>,
    /// Instrument effects.
    effect: Vec<u32>,
    //    ///
    //    sounds: Vec<Sound>,
    /// Volume: 0-1
    volume: f32,
}

/// Synthesis file.
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Synth {
    /// Instrument presets,
    /// Reverb presets, IDs automatically assigned.
    effect: Vec<Effect>,
    /// Channels
    chan: Vec<SynthChan>,
}

/// A signature.
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Sig {
    /// The key signature (0-23 quarter steps above C, 24+ reserved for middle
    /// eastern and Indian key signatures).
    pub key: u8,
    /// Time signature (num_beats/note_len), 4/4 is common.
    pub time: String,
    /// BPM (beats per minute), 120 BPM is common (default=120).
    pub tempo: u16,
    /// % Swing (default=50).
    pub swing: Option<u8>,
}

/// Channel information for a specific bar of music.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Chan {
    /// Channel notes for 1 bar.
    notes: String,
    /// Channel lyrics for 1 bar.
    lyric: Option<String>,
}

/// A parsed and transformed channel information for a specific bar of music.
#[derive(PartialEq, Debug)]
pub struct Channel {
    /// Channel notes for 1 bar.
    notes: Vec<Marking>,
    /// Channel lyrics for 1 bar.
    lyric: Option<String>,
}

impl Default for Chan {
    fn default() -> Self {
        let notes = String::new(); // no notes = whole measure rest
        let lyric = None;
        Chan { notes, lyric }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Chan::default().into()
    }
}

impl From<Chan> for Channel {
    fn from(chan: Chan) -> Self {
        let mut notes = vec![];

        for marking in chan.notes.split(' ').filter(|m| !m.is_empty()) {
            notes.push(marking.parse().unwrap_or_else(|_| {
                panic!("Invalid marking: {}", marking);
            }));
        }

        let lyric = chan.lyric;

        Channel { notes, lyric }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SigRef {
    /// Index into sig list.
    index: u32,
    /// Which beat of the measure the signature starts applying to.
    beat: Option<u8>,
}

/// A bar (or measure) of music.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    /// Signature reference (index)
    sig: Option<SigRef>,
    /// All of the channels in this piece.
    chan: Vec<Chan>,
    /// Repeat symbols for this measure.
    repeat: Vec<String>,
}

/// A bar (or measure) of music.
#[derive(Debug, PartialEq)]
pub struct Measure {
    /// Signature reference (index)
    pub sig: Option<SigRef>,
    /// All of the channels in this piece.
    pub chan: Vec<Channel>,
    /// Repeat symbols for this measure.
    pub repeat: Vec<String>,
}

impl From<Bar> for Measure {
    fn from(mut bar: Bar) -> Self {
        let mut chan = vec![];

        for i in bar.chan.drain(..) {
            chan.push(i.into());
        }

        let sig = bar.sig;
        let repeat = bar.repeat;

        Measure { sig, chan, repeat }
    }
}

/// A movement in the score.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Mvmt {
    /// A list of key signatures used in this movement.
    pub sig: Vec<Sig>,
    /// Each measure of the movement in order.
    pub bar: Vec<Bar>,
}

impl Default for Mvmt {
    fn default() -> Mvmt {
        muon::from_str(include_str!("default_movement.muon")).unwrap()
    }
}

/// A movement in the score.
#[derive(PartialEq, Debug)]
pub struct Movement {
    /// A list of key signatures used in this movement.
    pub sig: Vec<Sig>,
    /// Each measure of the movement in order.
    pub bar: Vec<Measure>,
}

impl Default for Movement {
    fn default() -> Movement {
        Mvmt::default().into()
    }
}

impl From<Mvmt> for Movement {
    fn from(mut mvmt: Mvmt) -> Movement {
        let sig = mvmt.sig;
        let mut bar = Vec::new();

        bar.extend(mvmt.bar.drain(..).map(|i| i.into()));

        Movement { sig, bar }
    }
}

/// An instrument in the soundfont for this score.
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Instrument {
    // Default waveform for instrument.
    waveform: String,
    // Straight or Palm mute depending on instrument.
    mute: Option<String>,
    // Cup mute.
    cup_mute: Option<String>,
    // Wah-wah mute.
    harmon_mute: Option<String>,
    // Plunger mute.
    plunger_mute: Option<String>,
    // Harmonic (for guitar)
    harmonic: Option<String>,

    // Use different waveform for this dynamic
    ppp: Option<String>,
    // Use different waveform for this dynamic
    pp: Option<String>,
    // Use different waveform for this dynamic
    p: Option<String>,
    // Use different waveform for this dynamic
    mp: Option<String>,
    // Use different waveform for this dynamic
    mf: Option<String>,
    // Use different waveform for this dynamic
    f: Option<String>,
    // Use different waveform for this dynamic
    ff: Option<String>,
    // Use different waveform for this dynamic
    fff: Option<String>,
}

/*/// A soundfont used in the score (either in the .scof or a .sfsf and linked to).
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SoundFont {
    /// A list of instruments.
    pub instrument: Vec<Instrument>,
}*/

fn default_symtime() -> bool {
    false // Don't show a symbol for time signature.
}

/// Signature Style.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct SigStyle {
    /// Text that should show up.  Default="beat = BPM" marking.
    pub tempo: Option<String>,
    /// Whether or not the time signature should use a special symbol
    /// (C for 4/4).
    #[serde(default = "default_symtime")]
    pub time_symbol: bool,
    /// Text that should show up rather than default.
    /// Default="1/8 1/8 = 1/6 1/12"
    pub swing_text: Option<String>,
}

/// Style file.
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Style {
    pub sig: Vec<SigStyle>,
}

/// Arranger & Ensemble
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Arranger {
    name: String,
    ensemble: Option<String>,
}

fn default_composer() -> String {
    "Anonymous".to_string()
}

/// Score metadata.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Meta {
    /// Who wrote the original music "{}"
    #[serde(default = "default_composer")]
    pub composer: String,
    /// The subtitle of the piece.
    pub subtitle: Option<String>,
    /// Work number.
    pub number: Option<u32>,
    /// Who wrote the lyrics to the music "Words by {}"
    pub lyricist: Option<String>,
    /// Who translated the lyrics "Translated by {}"
    pub translator: Option<String>,
    /// Who performed the music "Performed by {}"
    pub performers: Option<String>,
    /// List of people who arranged & rearranged the music in order
    /// "Arranged for {} by {}".
    pub arranger: Vec<Arranger>,
    /// List of people who revised the score "Revised by {}".
    pub revised: Vec<String>,
    /// License information
    pub licenses: Vec<String>,
    /// Playing level (how hard it is to play times 2 - to allow grade 1.5 etc.).
    pub grade: Option<u8>,
    /// List of the movements in order.
    pub movement: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            subtitle: None,
            number: None,
            composer: default_composer(),
            lyricist: None,
            translator: None,
            performers: None,
            arranger: vec![],
            revised: vec![],
            licenses: vec![],
            grade: None,
            movement: vec![],
        }
    }
}

/// The entire Scof zip file.
pub struct Scof {
    /// The title of the piece.  When the zip file's name is
    /// "My Score \ Symphony No. 1.scof" => "My Score / Symphony No. 1".
    /// Maximum of 64 characters.
    pub title: String,
    /// Bytes for an RVG file (Vector(SVG), Pixel(PNG) or Picture(JPG)).
    pub cover: Option<Vec<u8>>,
    /// Metadata for the peice.
    pub meta: Meta,
    /// Rendering style.
    pub style: Style,
    /// Playback synthesis.
    pub synth: Synth,
    /// Instruments.
    pub soundfont: Vec<Instrument>,
    /// Movements for the peice.
    pub movement: Vec<Movement>,

    /// Cache for time signatures of each measure in each movement.
    pub cache: Vec<Vec<Fraction>>,
}

impl Default for Scof {
    fn default() -> Scof {
        Scof {
            title: "Untitled Score".to_string(),
            cover: None,
            meta: Meta::default(),
            style: Style::default(),
            synth: Synth::default(),
            movement: vec![Movement::default()],
            soundfont: vec![Instrument::default()],

            cache: vec![vec![]],
        }
    }
}

impl Scof {
    /// Lookup a marking at a cursor position
    pub fn marking(&self, cursor: &Cursor) -> Option<&Marking> {
        self.movement
            .get(cursor.movement as usize)?
            .bar
            .get(cursor.measure as usize)?
            .chan
            .get(cursor.chan as usize)?
            .notes
            .get(cursor.marking as usize)
    }

    /// Get mutable marking at a cursor position
    pub fn marking_mut(&mut self, cursor: &Cursor) -> Option<&mut Marking> {
        self.chan_notes_mut(cursor)?
            .get_mut(cursor.marking as usize)
    }

    /// Get mutable vec of markings for measure at cursor position.
    fn chan_notes_mut(&mut self, cursor: &Cursor) -> Option<&mut Vec<Marking>> {
        Some(
            &mut self
                .movement
                .get_mut(cursor.movement as usize)?
                .bar
                .get_mut(cursor.measure as usize)?
                .chan
                .get_mut(cursor.chan as usize)?
                .notes,
        )
    }

    /// Get the last measure of a movement
    fn last_measure(&self, movement: usize) -> Option<&Measure> {
        self.movement.get(movement)?.bar.last()
    }

    /// Push a measure at end of movement
    fn push_measure(&mut self, movement: usize, bar: Measure) {
        if let Some(movement) = &mut self.movement.get_mut(movement) {
            movement.bar.push(bar);
        }
    }

    /// Add a new measure
    pub fn new_measure(&mut self) {
        if let Some(last_bar) = self.last_measure(0) {
            // Add whole rests for each channel.
            let mut chan = vec![];
            for _ in last_bar.chan.iter() {
                chan.push(Channel::default());
            }
            self.push_measure(
                0,
                Measure {
                    sig: None,      // No signature changes
                    repeat: vec![], // No repeat symbols
                    chan,
                },
            );
        }
    }

    /// Get the count of markings in a measure
    pub fn marking_len(&self, cursor: &Cursor) -> u16 {
        let mut curs = (*cursor).clone();
        curs.marking = 0;
        while self.marking(&curs).is_some() {
            curs.marking += 1;
        }
        curs.marking
    }

    /// Get the note at cursor
    pub fn note(&self, cursor: &Cursor) -> Option<&Note> {
        if let Marking::Note(note) = self.marking(cursor)? {
            Some(note)
        } else {
            None
        }
    }

    /// Set pitch class and octave of a note at a cursor
    pub fn set_pitch(&mut self, cursor: &Cursor, i: u16, pitch: Pitch) {
        let mut note = self.note(cursor).unwrap().clone();
        note.set_pitch(i, pitch);
        let m = self.marking_mut(cursor).unwrap();
        *m = Marking::Note(note);
    }

    /// Set an empty measure to be filled with all of the beats.
    /// Returns the fraction that doesn't fit in the measure.
    pub fn set_empty_measure(
        &mut self,
        cursor: &Cursor,
        note: &Note,
    ) -> Option<Fraction> {
        // FIXME: Time Signatures
        self.chan_notes_mut(cursor)
            .unwrap()
            .push("1/1R".parse().unwrap());
        self.set_full_measure(cursor, note)
    }

    /// Set a full measure to be replaced at the start.
    /// Returns the fraction that doesn't fit in the measure.
    pub fn set_full_measure(
        &mut self,
        cursor: &Cursor,
        note: &Note,
    ) -> Option<Fraction> {
        let mut cursor = cursor.clone();
        cursor.marking = 0;
        self.set_part_measure(&cursor, note)
    }

    /// Set a full measure to be replaced at the start.
    /// Returns the fraction that doesn't fit in the measure.
    pub fn set_part_measure(
        &mut self,
        cursor: &Cursor,
        note: &Note,
    ) -> Option<Fraction> {
        let mut note = note.clone();

        let notes = self.chan_notes_mut(cursor).unwrap();
        let mut new_notes = vec![];
        let mut quota = note.duration;

        let b = cursor.marking;

        new_notes.extend(notes.drain(0..b.try_into().unwrap()));

        let mut i = 0;
        i = loop {
            if i == notes.len() {
                cala::note!("END {} {}", note.duration, quota);
                note.duration -= quota;
                new_notes.push(Marking::Note(note));
                *notes = new_notes;
                return Some(quota);
            }

            let marking: &Marking = &notes[i];

            i += 1;

            // Only count notes.
            if let Marking::Note(parsed_note) = marking {
                if quota > parsed_note.duration {
                    quota -= parsed_note.duration;
                } else if quota == parsed_note.duration {
                    new_notes.push(Marking::Note(note));
                    break i;
                } else {
                    let mut modified = parsed_note.clone();
                    modified.duration -= quota;
                    new_notes.push(Marking::Note(note));
                    new_notes.push(Marking::Note(modified));
                    break i;
                }
            }
        };

        new_notes.extend(notes.drain(i..notes.len()));
        *notes = new_notes;

        None
    }

    /// Set whole rest at cursor to C4.
    pub fn set_whole_pitch(&mut self, cursor: &Cursor) {
        // If it's a whole measure rest, insert a whole note (4/4)
        // FIXME: Add time signatures.
        self.chan_notes_mut(cursor)
            .unwrap()
            .push("1/1C4".parse().unwrap());
    }

    /// Set duration of a note.
    pub fn set_duration(&mut self, cursor: &Cursor, dur: Fraction) {
        let mut note = self.note(cursor).unwrap().clone();
        let old = note.duration;
        note.set_duration(dur);
        if old > dur {
            let rests = old - dur;

            self.insert_after(
                cursor,
                Marking::Note(Note {
                    pitch: vec![],
                    duration: rests,
                    articulation: vec![],
                }),
            );

            // Set first note.
            let m = self.marking_mut(&cursor).unwrap();
            *m = Marking::Note(note);
        } else {
            let mut cursor = cursor.clone();

            while let Some(rem) = self.set_part_measure(&cursor, &note) {
                cala::note!("Remainder {}", rem);
                cursor.measure += 1;
                cursor.marking = 0;
                self.new_measure();
                let notes = self.chan_notes_mut(&cursor).unwrap();
                if notes.is_empty() {
                    notes.push("1/1R".parse().unwrap());
                }
                note.set_duration(rem);
                cala::note!("Next…");
            }
            cala::note!("RFni");
        }
    }

    pub fn set_whole_duration(&mut self, cursor: &Cursor, dur: Fraction) {
        let note = Note {
            pitch: vec![],
            duration: dur,
            articulation: vec![],
        };

        self.set_empty_measure(cursor, &note);
    }

    // FIXME: Needed?
    /// Insert a note after the cursor.
    fn insert_after(
        &mut self,
        cursor: &Cursor,
        marking: Marking,
    ) -> Option<()> {
        let _string = self
            .chan_notes_mut(&cursor.clone().right_unchecked())?
            .insert((cursor.marking + 1).try_into().unwrap(), marking);
        Some(())
    }

    /// Insert a note after the cursor.
    fn insert_at(&mut self, cursor: &Cursor, marking: Marking) -> Option<()> {
        self.chan_notes_mut(cursor)?
            .insert(cursor.marking.try_into().unwrap(), marking);
        Some(())
    }

    /// Remove the note after the cursor.
    fn remove_at(&mut self, cursor: &Cursor) -> Option<Marking> {
        let notes = self.chan_notes_mut(cursor)?;
        let note = notes.remove(cursor.marking.try_into().unwrap());

        cala::note!("REMOVE \"{:?}\"", note);

        Some(note)
    }
}
