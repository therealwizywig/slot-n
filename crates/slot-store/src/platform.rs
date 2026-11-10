use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum Platform {
    #[default]
    Gba,
    Gb,
    Gbc,
    Gbav,
    Nes,
    Snes,
    MediaPlayer,
    #[cfg(feature = "nds")]
    Nds,
}

impl Platform {
    #[cfg(not(feature = "nds"))]
    pub const ALL: [Platform; 7] = [
        Platform::Gba,
        Platform::Gbav,
        Platform::Gbc,
        Platform::Gb,
        Platform::Nes,
        Platform::Snes,
        Platform::MediaPlayer,
    ];
    #[cfg(feature = "nds")]
    pub const ALL: [Platform; 8] = [
        Platform::Nds,
        Platform::Gba,
        Platform::Gbav,
        Platform::Gbc,
        Platform::Gb,
        Platform::Nes,
        Platform::Snes,
        Platform::MediaPlayer,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Platform::Gba => "Game Boy Advance",
            Platform::Gb => "Game Boy",
            Platform::Gbc => "Game Boy Color",
            Platform::Gbav => "Game Boy Advance Video",
            Platform::Nes => "Nintendo Entertainment System",
            Platform::Snes => "Super Nintendo",
            Platform::MediaPlayer => "Video",
            #[cfg(feature = "nds")]
            Platform::Nds => "Nintendo DS",
        }
    }

    pub fn short_name(self) -> &'static str {
        match self {
            Platform::Nes => "NES",
            Platform::Snes => "SNES",
            #[cfg(feature = "nds")]
            Platform::Nds => "NDS",
            other => other.name(),
        }
    }

    pub fn dir_name(self) -> &'static str {
        match self {
            Platform::Gba => "GBA",
            Platform::Gb => "GB",
            Platform::Gbc => "GBC",
            Platform::Gbav => "GBAV",
            Platform::Nes => "NES",
            Platform::Snes => "SNES",
            Platform::MediaPlayer => "MEDIA",
            #[cfg(feature = "nds")]
            Platform::Nds => "NDS",
        }
    }

    pub fn hardware(self) -> Platform {
        match self {
            Platform::Gbav => Platform::Gba,
            p => p,
        }
    }

    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Platform::Gba | Platform::Gbav => &["gba"],
            Platform::Gb | Platform::Gbc => &["gb", "gbc"],
            Platform::Nes => &["nes", "unf", "unif"],
            Platform::Snes => &["sfc", "smc"],
            Platform::MediaPlayer => &["mp4", "mkv", "avi", "webm", "mov", "m4v"],
            #[cfg(feature = "nds")]
            Platform::Nds => &["nds"],
        }
    }

    pub fn picture(self) -> (u32, u32) {
        match self {
            Platform::Gba | Platform::Gbav => (240, 160),
            Platform::Gb | Platform::Gbc => (160, 144),
            Platform::Nes => (256, 240),
            Platform::Snes => (256, 224),
            Platform::MediaPlayer => (640, 480),
            #[cfg(feature = "nds")]
            Platform::Nds => (256, 192),
        }
    }

    pub fn accepts(self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| {
                self.extensions()
                    .iter()
                    .any(|k| ext.eq_ignore_ascii_case(k))
            })
    }
}
