//!

use std::{fmt, str::FromStr};

/// An articulation (affects how the note is played).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Articulation {
    /// Really separated.
    Staccatissimo,
    /// Separated (short 1/2)
    Staccato,
    /// Tenuto
    Tenuto,
    /// Marcato (short sharp attack) (2/3)
    Marcato,
    /// Accent (sharp attack)
    Accent,

    /// Closed mute (or palm mute rendered as _ on guitar)
    Mute,
    /// Open (no) Mute
    Open,
    /// Harmonic
    Harmonic,
    /// Pedal
    Pedal,

    /// Slur
    Slur,
    /// Glissando
    Glissando,
    /// Pitch bend slide up into
    BendUpInto,
    /// Pitch bend slide down into
    BendDownInto,
    /// Pitch bend slide up out of
    BendUpOut,
    /// Pitch bend slide down out of (fall)
    BendDownOut,

    /// Turn
    Turn,
    /// Inverted Turn
    TurnInverted,
    /// Trill
    Trill,
    /// Tremelo
    Tremelo,
    /// Arpeggio (strum) pitch up, strum guitar down.
    StrumDown,
    /// Arpeggio (strum) pitch down, strum guitar up
    StrumUp,

    /// Fermata (everyone plays long)
    Fermata,
}

impl fmt::Display for Articulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Articulation::*;

        match self {
            // Articulation
            Staccatissimo => write!(f, "'"),
            Staccato => write!(f, "."),
            Tenuto => write!(f, "_"),
            Marcato => write!(f, "^"),
            Accent => write!(f, ">"),
            // Sound modifiers
            Mute => write!(f, "+"),
            Open => write!(f, "o"),
            Harmonic => write!(f, "@"),
            Pedal => write!(f, "|"),
            // Connections between notes.
            Slur => panic!("Should this be articulation?"), // FIXME
            Glissando => panic!("Should this be articulation?"), // FIXME
            BendUpInto => panic!("Should this be articulation?"), // FIXME
            BendDownInto => panic!("Should this be articulation?"), // FIXME
            BendUpOut => panic!("Should this be articulation?"), // FIXME
            BendDownOut => panic!("Should this be articulation?"), // FIXME

            // Adds extra notes within one note.
            Turn => panic!("Should this be articulation?"), // FIXME
            TurnInverted => panic!("Should this be articulation?"), // FIXME
            Trill => panic!("Should this be articulation?"), // FIXME
            Tremelo => panic!("Should this be articulation?"), // FIXME
            StrumDown => panic!("Should this be articulation?"), // FIXME
            StrumUp => panic!("Should this be articulation?"), // FIXME

            // Applies to all staves at the same time.
            Fermata => panic!("Should this be articulation?"), // FIXME
        }
    }
}

impl FromStr for Articulation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().ok_or(())? {
            // Articulation
            '\'' => Articulation::Staccatissimo,
            '.' => Articulation::Staccato,
            '_' => Articulation::Tenuto,
            '^' => Articulation::Marcato,
            '>' => Articulation::Accent,
            // Sound modifiers
            '+' => Articulation::Mute,
            'o' => Articulation::Open,
            '@' => Articulation::Harmonic,
            '|' => Articulation::Pedal,
            _ => return Err(()),
        })
    }
}
