mod common;

use common::{app_booting_at, CLOCK_IS_SET};
use slot::app::{PageModel, Phase};
use slot::root::missing_bios;
use slot_input::{Action, Btn};
use slot_store::{write_slot_state, Platform, SlotState};

fn card_with(games: &[&str]) -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    slot::root::ensure(d.path());
    for g in games {
        let path = d.path().join("Games").join(g);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, b"PK").unwrap();
    }
    write_slot_state(
        d.path(),
        &SlotState {
            clock_set: true,
            ..SlotState::default()
        },
    )
    .unwrap();
    d
}

fn press(a: &mut slot::app::App, btn: Btn) {
    a.apply(Action::GbaDown(btn));
    a.apply(Action::GbaUp(btn));
}

#[test]
fn the_shelf_holds_the_game_boy_family_and_nes_snes_nds() {
    let d = card_with(&[
        "GBA/fusion.gba",
        "GB/tetris.gb",
        "GBC/pokemon.gbc",
        "NES/Zelda.nes",
        "SNES/Mario.sfc",
        "NDS/Mario.nds",
    ]);
    let (mut a, _clock) = app_booting_at(d.path(), CLOCK_IS_SET);
    let mut names = vec![a.shelf_platform_name().expect("a system on the shelf")];
    for _ in 0..8 {
        press(&mut a, Btn::R1);
        names.push(a.shelf_platform_name().expect("a system on the shelf"));
    }
    for want in [
        "Game Boy Advance",
        "Game Boy",
        "Game Boy Color",
        "Nintendo Entertainment System",
        "Super Nintendo",
        "Nintendo DS",
    ] {
        assert!(names.contains(&want), "{want} was not on the shelf: {names:?}");
    }
    assert!(
        !names.iter().any(|n| n.contains("Genesis") || *n == "Arcade"),
        "a system this fork does not ship sat on the shelf: {names:?}"
    );
}

#[test]
fn a_bios_is_found_under_any_of_its_names_or_beside_the_games() {
    let d = card_with(&[]);
    let bios = d.path().join("BIOS");
    assert_eq!(
        missing_bios(d.path(), Platform::SegaCd),
        Some("bios_CD_U.bin|bios_CD_E.bin|bios_CD_J.bin")
    );
    std::fs::write(bios.join("bios_CD_J.bin"), b"").unwrap();
    assert_eq!(missing_bios(d.path(), Platform::SegaCd), None);

    assert!(missing_bios(d.path(), Platform::NeoGeo).is_some());
    let neogeo = d.path().join("Games/NEOGEO");
    std::fs::create_dir_all(&neogeo).unwrap();
    std::fs::write(neogeo.join("neogeo.zip"), b"PK").unwrap();
    assert_eq!(missing_bios(d.path(), Platform::NeoGeo), None);

    // Both of the Intellivision's are needed.
    std::fs::write(bios.join("exec.bin"), b"").unwrap();
    assert_eq!(
        missing_bios(d.path(), Platform::Intellivision),
        Some("grom.bin")
    );
}

#[test]
fn every_system_has_its_own_folder_on_the_card() {
    let d = card_with(&[]);
    for p in Platform::SHELF {
        assert!(
            d.path().join("Games").join(p.dir_name()).is_dir(),
            "no Games/{}",
            p.dir_name()
        );
    }
    let dirs: Vec<&str> = Platform::SHELF.iter().map(|p| p.dir_name()).collect();
    for (i, d) in dirs.iter().enumerate() {
        assert!(!dirs[i + 1..].contains(d), "two systems share {d}");
    }
}

/// Types `word` on the keyboard by walking to each key, which starts on `q`.
fn type_word(a: &mut slot::app::App, word: &str) {
    const ROWS: [&str; 4] = ["1234567890", "qwertyuiop", "asdfghjkl-", "zxcvbnm_.@"];
    let (mut row, mut col) = (1usize, 0usize);
    for ch in word.chars() {
        let r = ROWS
            .iter()
            .position(|r| r.contains(ch))
            .expect("a key for it");
        let c = ROWS[r].find(ch).unwrap();
        while row < r {
            press(a, Btn::Down);
            row += 1;
        }
        while row > r {
            press(a, Btn::Up);
            row -= 1;
        }
        while col < c {
            press(a, Btn::Right);
            col += 1;
        }
        while col > c {
            press(a, Btn::Left);
            col -= 1;
        }
        press(a, Btn::A);
    }
}

#[test]
fn x_finds_a_game_on_any_shelf_and_a_takes_the_slot_to_it() {
    let d = card_with(&[
        "SNES/Super Metroid.sfc",
        "NES/Metroid.nes",
        "NES/Zelda.nes",
    ]);
    let (mut a, _clock) = app_booting_at(d.path(), CLOCK_IS_SET);
    press(&mut a, Btn::X);
    assert!(
        matches!(a.page_model(), Some(PageModel::Keys(_))),
        "X opened no keyboard: {:?}",
        a.phase()
    );
    type_word(&mut a, "met");
    press(&mut a, Btn::Start);
    let Some(PageModel::List(page)) = a.page_model() else {
        panic!("no results: {:?}", a.phase());
    };
    let found: Vec<(&str, &str)> = page
        .rows
        .iter()
        .map(|r| (r.label.as_str(), r.detail.as_str()))
        .collect();
    assert_eq!(
        found,
        [
            ("Metroid", "NES"),
            ("Super Metroid", "Super Nintendo"),
        ]
    );
    assert_eq!(page.status, "2 games match \u{201c}met\u{201d}");
    assert!(
        page.rows[0].icon.as_ref().is_some_and(|i| i.h > 0),
        "the first result has no picture"
    );
    press(&mut a, Btn::Down);
    press(&mut a, Btn::A);
    assert!(matches!(a.phase(), Phase::Shelf), "{:?}", a.phase());
    assert_eq!(a.selected_stem(), Some("Super Metroid"));

    // Y from the results searches again from what was typed; MENU goes back to them.
    press(&mut a, Btn::X);
    let Some(PageModel::Keys(keys)) = a.page_model() else {
        panic!("no keyboard");
    };
    assert_eq!(keys.text(), "met", "the last search was not kept");
}

#[test]
fn a_search_that_finds_nothing_says_so() {
    let d = card_with(&["NES/Zelda.nes", "NES/Metroid.nes"]);
    let (mut a, _clock) = app_booting_at(d.path(), CLOCK_IS_SET);
    press(&mut a, Btn::X);
    type_word(&mut a, "qq");
    press(&mut a, Btn::Start);
    let Some(PageModel::List(page)) = a.page_model() else {
        panic!("no results page");
    };
    assert!(page.rows.is_empty());
    assert_eq!(page.status, "No games match \u{201c}qq\u{201d}");
    press(&mut a, Btn::B);
    assert!(matches!(a.phase(), Phase::Shelf));
}
