//! VHS shell for the Media Player shelf. Neutral art is `assets/vhs_neutral.png`.

use std::sync::OnceLock;

use slot_store::Platform;

use crate::bitmaps;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Media {
    Vhs,
}

struct Art {
    png: &'static [u8],
    w: u32,
    h: u32,
    label: (u32, u32, u32, u32),
    colour: [u8; 3],
}

const ART: [Art; Media::COUNT] = [Art {
    png: include_bytes!("../../../assets/vhs_neutral.png"),
    w: 420,
    h: 236,
    label: (150, 72, 120, 88),
    colour: [0x2a, 0x2a, 0x2e],
}];

/// How tall a medium may sit in the slot: the Game Boy pak's seated height.
const SEATED_MAX_H: u32 = 259;

impl Media {
    pub const COUNT: usize = 1;
    pub const ALL: [Media; Media::COUNT] = [Media::Vhs];

    pub fn of(platform: Platform) -> Option<Media> {
        match platform {
            Platform::MediaPlayer => Some(Media::Vhs),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    fn art(self) -> &'static Art {
        &ART[self.index()]
    }

    pub fn size(self) -> (u32, u32) {
        (self.art().w, self.art().h)
    }

    pub fn label(self) -> (u32, u32, u32, u32) {
        self.art().label
    }

    pub fn colour(self) -> [u8; 3] {
        self.art().colour
    }

    pub fn seated(self, seated_w: u32) -> (u32, u32) {
        let (w, h) = self.size();
        let tall = (seated_w * h + w / 2) / w;
        if tall <= SEATED_MAX_H {
            return (seated_w, tall);
        }
        ((SEATED_MAX_H * w + h / 2) / h, SEATED_MAX_H)
    }

    pub fn neutral(self) -> &'static [u8] {
        static SHELLS: [OnceLock<Vec<u8>>; Media::COUNT] = [const { OnceLock::new() }; Media::COUNT];
        let art = self.art();
        bitmaps::load(art.png, art.w, art.h, &SHELLS[self.index()])
    }

    pub fn mask(self) -> &'static [u8] {
        static MASKS: [OnceLock<Vec<u8>>; Media::COUNT] = [const { OnceLock::new() }; Media::COUNT];
        MASKS[self.index()].get_or_init(|| self.neutral().iter().skip(3).step_by(4).copied().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_vhs_shell_decodes_and_its_label_fits() {
        let m = Media::Vhs;
        let (w, h) = m.size();
        assert_eq!(m.neutral().len(), (w * h * 4) as usize);
        let (lx, ly, lw, lh) = m.label();
        assert!(lx + lw <= w && ly + lh <= h);
        assert_eq!(Media::of(Platform::MediaPlayer), Some(Media::Vhs));
        assert!(Media::of(Platform::Gba).is_none());
        let (_, sh) = m.seated(240);
        assert!(sh <= SEATED_MAX_H);
    }
}
