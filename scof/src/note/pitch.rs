use std::fmt;
use std::str::FromStr;

use crate::note::Steps;

/// A Pitch Name.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PitchName {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
    G = 4,
    A = 5,
    B = 6,
}

impl fmt::Display for PitchName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use PitchName::*;

        match self {
            C => write!(f, "C"),
            D => write!(f, "D"),
            E => write!(f, "E"),
            F => write!(f, "F"),
            G => write!(f, "G"),
            A => write!(f, "A"),
            B => write!(f, "B"),
        }
    }
}

impl FromStr for PitchName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "C" => PitchName::C,
            "D" => PitchName::D,
            "E" => PitchName::E,
            "F" => PitchName::F,
            "G" => PitchName::G,
            "A" => PitchName::A,
            "B" => PitchName::B,
            _ => return Err(())
        })
    }
}

/// A Pitch Accidental.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PitchAccidental {
    ///
    DoubleFlat,
    ///
    FlatQuarterFlat,
    ///
    Flat,
    ///
    QuarterFlat,
    ///
    Natural,
    ///
    QuarterSharp,
    ///
    Sharp,
    ///
    SharpQuarterSharp,
    ///
    DoubleSharp,
}

impl fmt::Display for PitchAccidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use PitchAccidental::*;

        match self {
            DoubleFlat => write!(f, "bb"),
            FlatQuarterFlat => write!(f, "db"),
            Flat => write!(f, "b"),
            QuarterFlat => write!(f, "d"),
            Natural => write!(f, "n"),
            QuarterSharp => write!(f, "t"),
            Sharp => write!(f, "#"),
            SharpQuarterSharp => write!(f, "t#"),
            DoubleSharp => write!(f, "x"),
        }
    }
}

impl FromStr for PitchAccidental {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bb" => PitchAccidental::DoubleFlat,
            "db" => PitchAccidental::FlatQuarterFlat,
            "b" => PitchAccidental::Flat,
            "d" => PitchAccidental::QuarterFlat,
            "n" => PitchAccidental::Natural,
            "t" => PitchAccidental::QuarterSharp,
            "#" => PitchAccidental::Sharp,
            "t#" => PitchAccidental::SharpQuarterSharp,
            "x" => PitchAccidental::DoubleSharp,
            _ => return Err(())
        })
    }
}

/// A Pitch Class
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PitchClass {
    pub name: PitchName,
    pub accidental: Option<PitchAccidental>,
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name)?;
        if let Some(ref accidental) = self.accidental {
            write!(f, "{}", accidental)?;
        }
        Ok(())
    }
}

impl FromStr for PitchClass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
/*        if s.is_empty() {
            Err(())
        } else if s.len() == 1 {*/
            Ok(PitchClass {
                name: s.parse()?,
                accidental: None,
            })
/*        } else {
            Ok(PitchClass {
                name: PitchName::from_str(s.get(..1).ok_or(())?)?,
                accidental: Some(PitchAccidental::from_str(s.get(1..).ok_or(())?)?),
            })
        }*/
    }
}

/// A Pitch Octave
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i8)]
pub enum PitchOctave {
    /// Octave -1
    Octave_ = -1,
    /// Octave 0
    Octave0 = 0,
    /// Octave 1
    Octave1 = 1,
    /// Octave 2
    Octave2 = 2,
    /// Octave 3
    Octave3 = 3,
    /// Octave 4
    Octave4 = 4,
    /// Octave 5
    Octave5 = 5,
    /// Octave 6
    Octave6 = 6,
    /// Octave 7
    Octave7 = 7,
    /// Octave 8
    Octave8 = 8,
    /// Octave 9
    Octave9 = 9,
}

impl PitchOctave {
    /// Calculate a lower octave.
    pub fn lower(self) -> Option<PitchOctave> {
        use PitchOctave::*;

        match self {
            Octave_ => None,
            Octave0 => Some(Octave_),
            Octave1 => Some(Octave0),
            Octave2 => Some(Octave1),
            Octave3 => Some(Octave2),
            Octave4 => Some(Octave3),
            Octave5 => Some(Octave4),
            Octave6 => Some(Octave5),
            Octave7 => Some(Octave6),
            Octave8 => Some(Octave7),
            Octave9 => Some(Octave8),
        }
    }

    /// Calculate a higher octave.
    pub fn raise(self) -> Option<PitchOctave> {
        use PitchOctave::*;

        match self {
            Octave_ => Some(Octave0),
            Octave0 => Some(Octave1),
            Octave1 => Some(Octave2),
            Octave2 => Some(Octave3),
            Octave3 => Some(Octave4),
            Octave4 => Some(Octave5),
            Octave5 => Some(Octave6),
            Octave6 => Some(Octave7),
            Octave7 => Some(Octave8),
            Octave8 => Some(Octave9),
            Octave9 => None,
        }
    }
}

impl fmt::Display for PitchOctave {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use PitchOctave::*;

        match self {
            Octave_ => write!(f, "-"),
            Octave0 => write!(f, "0"),
            Octave1 => write!(f, "1"),
            Octave2 => write!(f, "2"),
            Octave3 => write!(f, "3"),
            Octave4 => write!(f, "4"),
            Octave5 => write!(f, "5"),
            Octave6 => write!(f, "6"),
            Octave7 => write!(f, "7"),
            Octave8 => write!(f, "8"),
            Octave9 => write!(f, "9"),
        }
    }
}

impl FromStr for PitchOctave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().nth(0).ok_or(())? {
            '-' => PitchOctave::Octave_,
            '0' => PitchOctave::Octave0,
            '1' => PitchOctave::Octave1,
            '2' => PitchOctave::Octave2,
            '3' => PitchOctave::Octave3,
            '4' => PitchOctave::Octave4,
            '5' => PitchOctave::Octave5,
            '6' => PitchOctave::Octave6,
            '7' => PitchOctave::Octave7,
            '8' => PitchOctave::Octave8,
            '9' => PitchOctave::Octave9,
            _ => return Err(()),
        })
    }
}

/// Pitch Class & Octave
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pitch(pub PitchClass, pub PitchOctave);

impl Pitch {
    pub fn visual_distance(&self) -> Steps {
        // Calculate number of octaves from middle C (C4).
        let octaves = self.1 as i32 - 4;
        // Calculate number of steps from C within key.
        let steps = self.0.name as i32;

        // Calculate total number of steps from middle C.
        Steps { 0: steps + octaves * 7 }
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl FromStr for Pitch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pitch_class = s[0..s.len()-1].parse::<PitchClass>()?;

        // Get Pitch Octave
        let pitch_octave = s[s.len()-1..].parse::<PitchOctave>()?;

        Ok(Pitch(pitch_class, pitch_octave))
    }
}
