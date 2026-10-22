# slot-n

A skeuomorphic frontend for Anbernic H700 handhelds on [BaseOS](https://github.com/pvaibhav/BaseOS). Fork of [slot](https://github.com/BrandonKowalski/slot): the Game Boy family is as it was, with NES, SNES and Nintendo DS added.

**Shelf:** Game Boy, Game Boy Color, Game Boy Advance, NES, SNES, Nintendo DS.

**Also:** Wi-Fi, Bluetooth (controllers and headphones), HDMI out, analog sticks, and a wallpaper you choose per system.

## Systems

| System | Folder | Files | Core | BIOS |
|---|---|---|---|---|
| Game Boy Advance | `GBA` | `.gba` | mGBA, gpSP | `gba_bios.bin` (optional) |
| Game Boy | `GB` | `.gb`, `.gbc` | mGBA | |
| Game Boy Color | `GBC` | `.gb`, `.gbc` | mGBA | |
| Nintendo Entertainment System | `NES` | `.nes`, `.unf`, `.unif` | FCEUmm | |
| Super Nintendo | `SNES` | `.sfc`, `.smc` | Snes9x | |
| Nintendo DS | `NDS` | `.nds`, `.zip` | DraStic, melonDS | |

Games go in `Games/<Folder>`. Labels, saves and states use the same folder names.

Nintendo DS defaults to standalone DraStic. Put a 64-bit `drastic` or `drastic_v2522` in `System/nds/`. melonDS is the in-process fallback from the core list.

## Wallpapers

Each system has its own background. Open that system's shelf, then Settings → Wallpaper, and cycle until you like it. PNG or JPG files in `Wallpapers/` on the card show up in that list.

## Network and HDMI

- **Wi-Fi** with an on-screen keyboard, **Bluetooth** controllers and headphones.
- **HDMI** on H700 handhelds with a mini-HDMI port: plug in a TV for 1080p, unplug to return to the panel. The lid does not sleep the handheld while the picture is on the TV.

## Install

slot-n runs on BaseOS on TF1. Flash a `slot-n-<TARGET>-bN.img.zip` for your model, or copy a built `dist-device/` tree onto an existing BaseOS card. The device binary is still `System/slot`, which is what BaseOS launches.

Wrong TARGET (for example `rgsp` vs `rg35xxsp`) is a black screen, not a slot-n bug.

## Build

See [FORK.md](FORK.md).

## Licence

GPL-3.0, as slot. See [LICENSE](LICENSE).
