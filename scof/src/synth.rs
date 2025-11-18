// ScoreFall Ink - Music Composition Software
//
// Copyright (C) 2019-2025 Jeryn Aldaron Lau <aldaronlau@gmail.com>
// Copyright (C) 2019-2025 Doug P. Lau
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

use serde_derive::{Deserialize, Serialize};

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
