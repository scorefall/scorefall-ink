/// Indices of each glyph (grouping most common ones at the beginning to help
/// with caching.
#[repr(u16)]
#[derive(PartialEq, Debug)]
pub enum Glyph {
    /* Noteheads */
    NoteheadFill = 0x1,
    NoteheadHalf = 0x2,
    NoteheadWhole = 0x3,
    NoteheadDouble = 0x10,

    NoteheadFillX = 0x0,
    NoteheadHalfX = 0x11,
    NoteheadWholeX = 0x12,
    NoteheadDoubleX = 0x13,

    NoteheadFillTriangle = 0x14,
    NoteheadHalfTriangle = 0x15,
    NoteheadWholeTriangle = 0x16,
    NoteheadDoubleTriangle = 0x17,

    NoteheadFillDiamond = 0x18,
    NoteheadHalfDiamond = 0x19,
    NoteheadWholeDiamond = 0x1A,
    NoteheadDoubleDiamond = 0x1B,

    NoteheadFillSlash = 0x1C,
    NoteheadHalfSlash = 0x1D,
    NoteheadWholeSlash = 0x1E,
    NoteheadDoubleSlash = 0x1F,

    NoteheadFillSlashed = 0x20,
    NoteheadHalfSlashed = 0x21,
    NoteheadWholeSlashed = 0x22,
    NoteheadDoubleSlashed = 0x23,

    /* Accidentals */
    Flat = 0x4,
    Sharp = 0x5,
    Natural = 0x6,

    DoubleFlat = 0x24,
    DoubleSharp = 0x25,
    QuarterFlat = 0x26,
    QuarterSharp = 0x27,
    ThreeQuarterFlat = 0x28,
    ThreeQuarterSharp = 0x29,
    ThirdFlat = 0x2A,
    ThirdSharp = 0x2B,
    TwoThirdFlat = 0x2C,
    TwoThirdSharp = 0x2D,

    /* Flags */
    FlagUp8 = 0x7,
    FlagDown8 = 0x8,
    FlagUp16 = 0x9,
    FlagDown16 = 0xA,

    FlagUp32 = 0x30,
    FlagDown32 = 0x31,
    FlagUp64 = 0x32,
    FlagDown64 = 0x33,

    /* Rests */
    RestMulti = 0x5C,
    Rest1 = 0xB,
    Rest2 = 0xC,
    Rest4 = 0xD,
    Rest8 = 0xE,
    Rest16 = 0xF,
    Rest32 = 0x2E,
    Rest64 = 0x2F,

    /* Clefs */
    /// Alto Clef (Soprano, Mezzo-Soprano, Alto, Tenor, Baritone)
    ClefC = 0x34,
    /// Treble Clef (French Violin, Treble)
    ClefG = 0x35,
    /// Bass Clef (Baritone, Bass, Sub-Bass)
    ClefF = 0x36,
    /// Percussion (Neutral) Clef
    ClefN = 0x37,

    /// Octave Up/Down
    Clef8 = 0x38,
    /// 2 Octave Up/Down
    Clef15 = 0x39,

    /* Tab "clefs" */
    Tab4 = 0x3A,
    Tab6 = 0x3B,

    /* Dynamics */
    P = 0x3C,
    MP = 0x3D,
    MF = 0x3E,
    F = 0x3F,
    /// May be displayed as "r" in some fonts.
    S = 0x4D,
    Z = 0x4E,
    N = 0x4F,

    /* Time signatures */
    TimeSig0 = 0x40,
    TimeSig1 = 0x41,
    TimeSig2 = 0x42,
    TimeSig3 = 0x43,
    TimeSig4 = 0x44,
    TimeSig5 = 0x45,
    TimeSig6 = 0x46,
    TimeSig7 = 0x47,
    TimeSig8 = 0x48,
    TimeSig9 = 0x49,
    TimeSigCommon = 0x4A,
    TimeSigCut = 0x4B,
    TimeSigPlus = 0x4C,

    /* Repeats */
    RepeatSlash = 0x5D,
    RepeatUpDot = 0x5E,
    RepeatDownDot = 0x5F,

    /* Jumps */
    Coda = 0x5A,
    Segno = 0x5B,

    /* Tuplet */
    TupletColon = 0x60,
    Tuplet0 = 0x50,
    Tuplet1 = 0x51,
    Tuplet2 = 0x52,
    Tuplet3 = 0x53,
    Tuplet4 = 0x54,
    Tuplet5 = 0x55,
    Tuplet6 = 0x56,
    Tuplet7 = 0x57,
    Tuplet8 = 0x58,
    Tuplet9 = 0x59,

    /* Stem Modifiers */
    Tremelo1 = 0x61,
    Tremelo2 = 0x62,
    Tremelo3 = 0x63,
    Tremelo4 = 0x64,
    Tremelo5 = 0x65,
    BuzzRoll = 0x66,
    Damp = 0x67,
    HarpStringNoise = 0x68,
    RimShot = 0x69,
    BowBridge = 0x6A,
    BowTailpiece = 0x6B,

    Len = 0x6C,
}

impl From<Glyph> for u16 {
    fn from(g: Glyph) -> Self {
        g as u16
    }
}
