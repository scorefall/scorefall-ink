# Scorefall Font
Parse / build ScoreFall Fonts.

# ScoreFall Font Format
ScoreFall fonts contain the following data fields in order, compressed with
the zstandard algorithm:

```
sffonts_version: u16 (uncompressed)
font_name_size: u8
font_name_data: ...
# Non-glyph components (in thousandths of stave space)
stave_line_thickness: u32
stem_thickness: u32
ledger_line_thickness: u32
ledger_line_extension: u32
slur_endpoint_thickness: u32 # Also used for ties
slur_midpoint_thickness: u32 # Also used for ties
barline_thickness: u32
thick_barline_thickness: u32
barlines_space: u32 # space between two barlines
barline_repeatdot_space: u32 # space between barline and repeat dots
bracket_thickness: u32 # instrument grouping
subbracket_thickness: u32 # instrument subgrouping
hairpin_thickness: u32 # Cresc., Dim., hairpin thickness (pedal, octave, ending,
                       # lyric melisma, tuple brackets)
rehearsal_box_thickness: u32

# [Glyph components] - SVG paths, Order specified by this format¹
path_size: u16
path_data: ...
```
