use std::{
    convert::TryInto,
    io::{Read, Write},
};

use crate::glyph::Glyph;

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

impl Default for GlyphsBuilder {
    fn default() -> Self {
        Self::new()
    }
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

        for glyph in self.glyphs.iter() {
            output.push_str(glyph.as_ref().unwrap());
            output.push('\0');
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

/// Read one 16-bit value in LE byte order
fn read_u16<T: Read>(reader: &mut T) -> Result<u16, ReadError> {
    let mut buf = [0u8; 2];
    reader
        .read_exact(&mut buf)
        .map_err(|_| ReadError::UnexpectedEOF)?;
    Ok(u16::from_le_bytes(buf))
}

/// Write one 16-bit value in LE byte order
fn write_u16<T: Write>(writer: &mut T, value: u16) -> Result<(), WriteError> {
    writer
        .write_all(&value.to_le_bytes())
        .map_err(|_| WriteError::Prevented)
}

/// Read one 32-bit value in LE byte order
fn read_i32<T: Read>(reader: &mut T) -> Result<i32, ReadError> {
    let mut buf = [0u8; 4];
    reader
        .read_exact(&mut buf)
        .map_err(|_| ReadError::UnexpectedEOF)?;
    Ok(i32::from_le_bytes(buf))
}

/// Write one 32-bit value in LE byte order
fn write_i32<T: Write>(writer: &mut T, value: i32) -> Result<(), WriteError> {
    let bytes = writer
        .write(&value.to_le_bytes())
        .map_err(|_| WriteError::Prevented)?;
    // FIXME: turn this into a write error
    assert_eq!(bytes, 4);
    Ok(())
}

/// Read two X,Y positions in LE byte order
fn read_positions<T: Read>(reader: &mut T) -> Result<[[i32; 2]; 2], ReadError> {
    let x1 = read_i32(reader)?;
    let y1 = read_i32(reader)?;
    let x2 = read_i32(reader)?;
    let y2 = read_i32(reader)?;
    Ok([[x1, y1], [x2, y2]])
}

/// Write two X,Y positions in LE byte order
fn write_positions<T: Write>(
    writer: &mut T,
    pos: [[i32; 2]; 2],
) -> Result<(), WriteError> {
    let [[x1, y1], [x2, y2]] = pos;
    write_i32(writer, x1)?;
    write_i32(writer, y1)?;
    write_i32(writer, x2)?;
    write_i32(writer, y2)?;
    Ok(())
}

/// Read a short string (0-255 bytes)
fn read_short_str<T: Read>(reader: &mut T) -> Result<String, ReadError> {
    let mut buf = [0u8; 1];
    reader
        .read_exact(&mut buf)
        .map_err(|_| ReadError::UnexpectedEOF)?;
    let mut buf = vec![0; buf[0] as usize];
    reader
        .read_exact(&mut buf)
        .map_err(|_| ReadError::UnexpectedEOF)?;
    Ok(String::from_utf8(buf).map_err(|_| ReadError::InvalidText)?)
}

/// Write a short string (0-255 bytes)
fn write_short_str<T: Write>(
    writer: &mut T,
    val: &str,
) -> Result<(), WriteError> {
    let len = val
        .len()
        .try_into()
        .map_err(|_| WriteError::FontNameTooLong)?;
    writer
        .write_all(&[len])
        .map_err(|_| WriteError::Prevented)?;
    writer
        .write_all(val.as_bytes())
        .map_err(|_| WriteError::Prevented)?;
    Ok(())
}

impl SfFontMetadata {
    /// Write font data.
    pub fn write<T: Write>(
        &self,
        writer: &mut T,
        glyph_paths: &str,
    ) -> Result<(), WriteError> {
        // Header
        write_u16(writer, self.sffonts_version)?;
        // FIXME: Start Compression
        write_short_str(writer, &self.font_name)?;

        // Non-glyph components (in thousandths of stave space)
        write_i32(writer, self.stave_line_thickness)?;
        write_i32(writer, self.stem_thickness)?;
        write_i32(writer, self.ledger_line_thickness)?;
        write_i32(writer, self.ledger_line_extension)?;
        write_i32(writer, self.slur_endpoint_thickness)?;
        write_i32(writer, self.slur_midpoint_thickness)?;
        write_i32(writer, self.barline_thickness)?;
        write_i32(writer, self.thick_barline_thickness)?;
        write_i32(writer, self.barlines_space)?;
        write_i32(writer, self.barline_repeatdot_space)?;
        write_i32(writer, self.bracket_thickness)?;
        write_i32(writer, self.subbracket_thickness)?;
        write_i32(writer, self.hairpin_thickness)?;
        write_i32(writer, self.rehearsal_box_thickness)?;

        // Glyph Metadata (Quarter)
        write_positions(writer, self.notehead)?;
        write_positions(writer, self.notehead_x)?;
        write_positions(writer, self.notehead_diamond)?;
        write_positions(writer, self.notehead_triangle)?;
        write_positions(writer, self.notehead_slash)?;

        // Glyph Metadata (Half)
        write_positions(writer, self.notehead_half)?;
        write_positions(writer, self.notehead_half_x)?;
        write_positions(writer, self.notehead_half_diamond)?;
        write_positions(writer, self.notehead_half_triangle)?;
        write_positions(writer, self.notehead_half_slash)?;

        // Glyph Metadata (Whole)
        write_positions(writer, self.notehead_whole)?;
        write_positions(writer, self.notehead_whole_x)?;
        write_positions(writer, self.notehead_whole_diamond)?;
        write_positions(writer, self.notehead_whole_triangle)?;
        write_positions(writer, self.notehead_whole_slash)?;

        // Glyph Metadata (Double Whole Notes)
        write_positions(writer, self.notehead_double)?;
        write_positions(writer, self.notehead_double_x)?;
        write_positions(writer, self.notehead_double_diamond)?;
        write_positions(writer, self.notehead_double_triangle)?;
        write_positions(writer, self.notehead_double_slash)?;

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
        // Header
        let sffonts_version = read_u16(&mut reader)?;

        // FIXME: Start De-Compression
        let font_name = read_short_str(&mut reader)?;

        // Non-glyph components (in thousandths of stave space)
        let stave_line_thickness = read_i32(&mut reader)?;
        let stem_thickness = read_i32(&mut reader)?;
        let ledger_line_thickness = read_i32(&mut reader)?;
        let ledger_line_extension = read_i32(&mut reader)?;
        let slur_endpoint_thickness = read_i32(&mut reader)?;
        let slur_midpoint_thickness = read_i32(&mut reader)?;
        let barline_thickness = read_i32(&mut reader)?;
        let thick_barline_thickness = read_i32(&mut reader)?;
        let barlines_space = read_i32(&mut reader)?;
        let barline_repeatdot_space = read_i32(&mut reader)?;
        let bracket_thickness = read_i32(&mut reader)?;
        let subbracket_thickness = read_i32(&mut reader)?;
        let hairpin_thickness = read_i32(&mut reader)?;
        let rehearsal_box_thickness = read_i32(&mut reader)?;

        // Glyph Metadata (Quarter)
        let notehead = read_positions(&mut reader)?;
        let notehead_x = read_positions(&mut reader)?;
        let notehead_diamond = read_positions(&mut reader)?;
        let notehead_triangle = read_positions(&mut reader)?;
        let notehead_slash = read_positions(&mut reader)?;

        // Glyph Metadata (Half)
        let notehead_half = read_positions(&mut reader)?;
        let notehead_half_x = read_positions(&mut reader)?;
        let notehead_half_diamond = read_positions(&mut reader)?;
        let notehead_half_triangle = read_positions(&mut reader)?;
        let notehead_half_slash = read_positions(&mut reader)?;

        // Glyph Metadata (Whole)
        let notehead_whole = read_positions(&mut reader)?;
        let notehead_whole_x = read_positions(&mut reader)?;
        let notehead_whole_diamond = read_positions(&mut reader)?;
        let notehead_whole_triangle = read_positions(&mut reader)?;
        let notehead_whole_slash = read_positions(&mut reader)?;

        // Glyph Metadata (Double Whole Notes)
        let notehead_double = read_positions(&mut reader)?;
        let notehead_double_x = read_positions(&mut reader)?;
        let notehead_double_diamond = read_positions(&mut reader)?;
        let notehead_double_triangle = read_positions(&mut reader)?;
        let notehead_double_slash = read_positions(&mut reader)?;

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
