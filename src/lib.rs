use quick_xml;

/// Structure for the whole score.
pub struct Score {
    pub info: ScoreInfo,
    pub ensemble: EnsembleStave,
}

/// Information about the score.
pub struct ScoreInfo {
    /// The date in which it's released (top left).
    pub release: String,

    pub attribution: ScoreAttribution,
    /// Work Title
    pub title: String,
    /// Which album work is from
    pub album: String,
    /// Who wrote the lyrics
    pub lyricist: String,
    /// Work #
    pub work: String,
    /// The copyright year (default: current).
    pub year: String,
    /// The license (default: "All Rights Reserved").
    pub license: String,
    /// The owner of the copyright (default: composer).
    pub copyright: String,
}

///
pub struct ScoreAttribution {
    /// Who published the score to be available for others to view
    /// (default: top left).
    ///
    /// ```
    /// [Release Date]
    /// [Publisher Name]
    /// ```
    pub publisher: String,
    /// Who wrote the music (default: top right).
    ///
    /// ```
    /// [Composer Name]                     # Always first
    /// Performed by [Performer Name]       # Only if different from composer
    /// Adapted by [Arranger Name]          # 
    /// Lyrics by [Lyricist Name]           # 
    /// Translated by [Translator Name]     # 
    /// ```
    pub composers: Vec<String>,
    /// List from first to last in chain of derivatives.
    /// (default: bottom center)
    ///
    /// ```
    /// Copyright © [Copyright Year Range] [Copyright Holder]
    /// All Rights Reserved.
    ///
    /// CC BY [Copyright Year Range] [Copyright Holder]
    /// This work is licensed under a Creative Commons Attribution 4.0
    /// International License.
    /// ```
    pub licenses: Vec<License>,
}

/// Different Licenses for Scores
pub enum License {
    /// No Sharing, No Adaptations, No Selling (Year, Rights Holder)
    AllRightsReserved(u16, String),
    /// Can Copy, Share, & Perform with Attribution, No Adaptations, No Selling
    CcByNcNd(u16, String),
    /// Can Copy, Share, & Perform with Attribution, No Adaptations, Can sell
    CcByNd(u16, String),
    /// Can Copy, Share, & Perform with Attribution, Can Adapt, No Selling
    CcByNc(u16, String),
    /// Can Copy, Share, & Perform with Attribution. Can Adapt, Can Sell
    CcBy(u16, String),
    /// Can Copy, Share, & Perform with Attribution, No Adaptations, No Selling,
    /// Share Alike
    CcByNcNdSa(u16, String),
    /// Can Copy, Share, & Perform with Attribution, No Adaptations, Can sell,
    /// Share Alike
    CcByNdSa(u16, String),
    /// Can Copy, Share, & Perform with Attribution, Can Adapt, No Selling,
    /// Share Alike
    CcByNcSa(u16, String),
    /// Can Copy, Share, & Perform with Attribution. Can Adapt, Can Sell,
    /// Share Alike
    CcBySa(u16, String),
    /// Can Copy, Share, & Perform without Attribution. Can Adapt, Can Sell
    Cc0PublicDomain,
}

/// All of the section staves in the score.
pub struct EnsembleStave {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
