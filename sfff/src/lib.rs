//! ScoreFall Font Format

use std::convert::TryInto;
use std::io::{Read, Write};

/// The number of units per stave space in this format.
pub const STAVE_SPACE: i32 = 100;

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

/// Create defs section of SVG for string of glyphs.
pub fn generate_defs(glyphs: &str) -> String {
    const HEADER: &str = "<defs>";
    const FOOTER: &str = "</defs>";

    // At least as much space will be needed.
    let output = Vec::with_capacity(glyphs.len() + HEADER.len() + FOOTER.len());
    let mut writer = std::io::BufWriter::new(std::io::Cursor::new(output));

    // Write to Vec should always succeed except on out of memory.
    let _ = write!(writer, "{}", HEADER);

    let mut id = 0;
    for glyph in glyphs.split('\0') {
        // Write to Vec should always succeed except on out of memory.
        let _ = write!(writer, "<path id=\"{:x}\" d=\"{}\"/>", id, glyph);
        id += 1;
    }

    assert_eq!(id, Glyph::Len as usize);

    // Unwrap: Write to Vec should always succeed except on out of memory.
    let _ = write!(writer, "{}", FOOTER);

    // 2 unwraps: Guaranteed to flush OK, and UTF-8 will always be valid.
    String::from_utf8(writer.into_inner().unwrap().into_inner()).unwrap()
}

/// Builder for all of the glyphs.
pub struct GlyphsBuilder {
    glyphs: Vec<Option<String>>,
}

impl GlyphsBuilder {
    pub fn new() -> Self {
        Self {
            glyphs: vec![None; Glyph::Len as usize],
        }
    }

    /// Add an SVG path.  Must be added in order.
    pub fn push(&mut self, glyph: Glyph, path: String) {
        self.glyphs[glyph as usize] = Some(path);
    }

    pub fn into_string(self) -> String {
        let mut output = String::new();

        let mut count = 0;
        for glyph in self.glyphs.iter() {
            output.push_str(glyph.as_ref().expect(&format!("!! {:X}", count)));
            output.push('\0');
            count += 1;
        }
        // Leave off the last null byte.
        output.pop();

        output
    }
}

/// Error for writing the format.
#[derive(Debug)]
pub enum WriteError {
    /// System prevented write for some reason.
    Prevented,
    /// Font name is too long.
    FontNameTooLong,
}

/// Error for reading the format.
#[derive(Debug)]
pub enum ReadError {
    /// System prevented read for some reason.
    Prevented,
    /// Invalid UTF-8
    InvalidText,
    /// Unexpected End-Of-File
    UnexpectedEOF,
    /// Wrong number of glyphs are in the file.
    WrongGlyphCount,
}

/// A ScoreFall Font Metadata
pub struct SfFontMetadata {
    /// Must be 0
    pub sffonts_version: u16,
    /// Name of this font
    pub font_name: String,

    // Non-glyph components (in thousandths of stave space)
    ///
    pub stave_line_thickness: i32,
    ///
    pub stem_thickness: i32,
    ///
    pub ledger_line_thickness: i32,
    ///
    pub ledger_line_extension: i32,
    /// Also used for ties
    pub slur_endpoint_thickness: i32,
    /// Also used for ties
    pub slur_midpoint_thickness: i32,
    ///
    pub barline_thickness: i32,
    ///
    pub thick_barline_thickness: i32,
    /// Space between two barlines
    pub barlines_space: i32,
    /// Space between barline and repeat dots
    pub barline_repeatdot_space: i32,
    /// Instrument grouping
    pub bracket_thickness: i32,
    /// Instrument subgrouping
    pub subbracket_thickness: i32,
    /// Cresc., Dim., hairpin thickness (pedal, octave, ending, lyric melisma,
    /// tuple brackets)
    pub hairpin_thickness: i32,
    ///
    pub rehearsal_box_thickness: i32,

    // Glyph metadata (Notehead & Stem Positions)
    pub notehead: [[i32; 2]; 2], // also includes slashed notehead
    pub notehead_x: [[i32; 2]; 2],
    pub notehead_diamond: [[i32; 2]; 2],
    pub notehead_triangle: [[i32; 2]; 2],
    pub notehead_slash: [[i32; 2]; 2],

    pub notehead_half: [[i32; 2]; 2], // also includes slashed notehead
    pub notehead_half_x: [[i32; 2]; 2],
    pub notehead_half_diamond: [[i32; 2]; 2],
    pub notehead_half_triangle: [[i32; 2]; 2],
    pub notehead_half_slash: [[i32; 2]; 2],

    pub notehead_whole: [[i32; 2]; 2], // also includes slashed notehead
    pub notehead_whole_x: [[i32; 2]; 2],
    pub notehead_whole_diamond: [[i32; 2]; 2],
    pub notehead_whole_triangle: [[i32; 2]; 2],
    pub notehead_whole_slash: [[i32; 2]; 2],

    pub notehead_double: [[i32; 2]; 2], // also includes slashed notehead
    pub notehead_double_x: [[i32; 2]; 2],
    pub notehead_double_diamond: [[i32; 2]; 2],
    pub notehead_double_triangle: [[i32; 2]; 2],
    pub notehead_double_slash: [[i32; 2]; 2],
}

impl SfFontMetadata {
    /// Write font data.
    pub fn write<T: Write>(
        &self,
        writer: &mut T,
        glyph_paths: &str,
    ) -> Result<(), WriteError> {
        // Header
        writer
            .write(&self.sffonts_version.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        // FIXME: Start Compression
        writer
            .write(&[self
                .font_name
                .len()
                .try_into()
                .map_err(|_| WriteError::FontNameTooLong)?])
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(self.font_name.as_bytes())
            .map_err(|_| WriteError::Prevented)?;

        // Non-glyph components (in thousandths of stave space)
        writer
            .write(&self.stave_line_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.stem_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.ledger_line_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.ledger_line_extension.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.slur_endpoint_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.slur_midpoint_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.barline_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.thick_barline_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.barlines_space.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.barline_repeatdot_space.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.bracket_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.subbracket_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.hairpin_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.rehearsal_box_thickness.to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;

        // Glyph
        writer
            .write(&self.notehead[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_x[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_x[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_x[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_x[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_diamond[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_diamond[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_diamond[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_diamond[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_triangle[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_triangle[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_triangle[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_triangle[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_slash[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_slash[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_slash[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_slash[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;

        writer
            .write(&self.notehead_half[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_x[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_x[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_x[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_x[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_diamond[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_diamond[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_diamond[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_diamond[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_triangle[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_triangle[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_triangle[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_triangle[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_slash[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_slash[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_slash[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_half_slash[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;

        writer
            .write(&self.notehead_whole[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_x[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_x[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_x[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_x[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_diamond[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_diamond[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_diamond[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_diamond[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_triangle[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_triangle[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_triangle[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_triangle[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_slash[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_slash[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_slash[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_whole_slash[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;

        writer
            .write(&self.notehead_double[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_x[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_x[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_x[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_x[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_diamond[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_diamond[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_diamond[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_diamond[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_triangle[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_triangle[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_triangle[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_triangle[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_slash[0][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_slash[0][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_slash[1][0].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;
        writer
            .write(&self.notehead_double_slash[1][1].to_le_bytes())
            .map_err(|_| WriteError::Prevented)?;

        // Glyph SVG paths
        writer
            .write(glyph_paths.as_bytes())
            .map_err(|_| WriteError::Prevented)?;

        // Make sure everything was written.
        writer.flush().map_err(|_| WriteError::Prevented)
    }

    /// Read a font into a metadata struct and a defs section of an SVG.
    pub fn from_buf_reader<T: Read>(
        mut reader: T,
    ) -> Result<(Self, String), ReadError> {
        let mut byte = [0u8; 1];
        let mut word = [0u8; 2];
        let mut long = [0u8; 4];

        // Header
        reader
            .read_exact(&mut word)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let sffonts_version = u16::from_le_bytes(word);

        // FIXME: Start De-Compression
        reader
            .read_exact(&mut byte)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let mut font_name = vec![0; byte[0] as usize];
        reader
            .read_exact(&mut font_name)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let font_name =
            String::from_utf8(font_name).map_err(|_| ReadError::InvalidText)?;

        // Non-glyph components (in thousandths of stave space)
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let stave_line_thickness = u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let stem_thickness = u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let ledger_line_thickness =
            u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let ledger_line_extension =
            u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let slur_endpoint_thickness =
            u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let slur_midpoint_thickness =
            u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let barline_thickness = u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let thick_barline_thickness =
            u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let barlines_space = u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let barline_repeatdot_space =
            u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let bracket_thickness = u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let subbracket_thickness = u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let hairpin_thickness = u32::from_le_bytes(long).try_into().unwrap();
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let rehearsal_box_thickness =
            u32::from_le_bytes(long).try_into().unwrap();

        // Glyph Metadata (Quarter)
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_x = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_diamond = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_triangle = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_slash = [[x1, y1], [x2, y2]];

        // Glyph Metadata (Half)
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_half = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_half_x = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_half_diamond = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_half_triangle = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_half_slash = [[x1, y1], [x2, y2]];

        // Glyph Metadata (Whole)
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_whole = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_whole_x = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_whole_diamond = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_whole_triangle = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_whole_slash = [[x1, y1], [x2, y2]];

        // Glyph Metadata (Double Whole Notes)
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_double = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_double_x = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_double_diamond = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_double_triangle = [[x1, y1], [x2, y2]];

        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y1 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let x2 = i32::from_le_bytes(long);
        reader
            .read_exact(&mut long)
            .map_err(|_| ReadError::UnexpectedEOF)?;
        let y2 = i32::from_le_bytes(long);
        let notehead_double_slash = [[x1, y1], [x2, y2]];

        // Glyph SVG paths
        let mut glyph_paths = String::new();
        reader
            .read_to_string(&mut glyph_paths)
            .map_err(|_| ReadError::Prevented)?;

        let new = Self {
            sffonts_version,
            font_name,
            stave_line_thickness,
            stem_thickness,
            ledger_line_thickness,
            ledger_line_extension,
            slur_endpoint_thickness,
            slur_midpoint_thickness,
            barline_thickness,
            thick_barline_thickness,
            barlines_space,
            barline_repeatdot_space,
            bracket_thickness,
            subbracket_thickness,
            hairpin_thickness,
            rehearsal_box_thickness,
            notehead, // also includes slashed notehead
            notehead_x,
            notehead_diamond,
            notehead_triangle,
            notehead_slash,
            notehead_half, // also includes slashed notehead
            notehead_half_x,
            notehead_half_diamond,
            notehead_half_triangle,
            notehead_half_slash,
            notehead_whole, // also includes slashed notehead
            notehead_whole_x,
            notehead_whole_diamond,
            notehead_whole_triangle,
            notehead_whole_slash,
            notehead_double, // also includes slashed notehead
            notehead_double_x,
            notehead_double_diamond,
            notehead_double_triangle,
            notehead_double_slash,
        };

        Ok((new, glyph_paths))
    }
}
