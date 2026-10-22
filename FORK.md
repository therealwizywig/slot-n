# slot-n

A focused fork of [slot](https://github.com/BrandonKowalski/slot) / [unslot](https://github.com/cloudwithax/unslot).

The Game Boy, Game Boy Color and Game Boy Advance shelves are unchanged. This tree adds NES, SNES and Nintendo DS, plus Wi-Fi, Bluetooth controllers, HDMI, and a wallpaper you pick per system.

| Piece | Source |
|---|---|
| GB, GBC, GBA | [BrandonKowalski/slot](https://github.com/BrandonKowalski/slot) |
| NES, SNES, Wi-Fi, Bluetooth, controllers | [cloudwithax/unslot](https://github.com/cloudwithax/unslot) |
| HDMI (sunxi disp2, 1280×720 → 1080p60, audio via `ahubhdmi`) | [nelynes/slotPlus](https://github.com/nelynes/slotPlus) |
| Nintendo DS (DraStic on device, melonDS as the in-process fallback) | this fork |
| Per-system wallpapers | this fork |

The extra unslot systems stay in the code so GB/GBC/GBA paths are not rewritten. They are not scanned and they are not on the shelf.

## HDMI

Switches before EGL starts. A cable change saves and relaunches so the Mali driver can rebuild the window. Closing the lid does nothing while the picture is on the TV.

## Nintendo DS

Default is standalone DraStic. The binary is proprietary and is not in the tree — put a 64-bit `drastic` or `drastic_v2522` in `System/nds/`. `libadvdrastic.so` and device profiles ship in `card/System/nds/`. melonDS is the in-process fallback (pick it from the core list). Logs: `Logs/NDS.txt`.

## Wallpapers

Settings → Wallpaper cycles the picture for the system whose shelf you are on. Each of GB, GBC, GBA, NES, SNES and NDS keeps its own pick in `Config/slot.state` (`wallpaper_gb=`, `wallpaper_nes=`, …). Put PNG or JPG files in `Wallpapers/` on the card.

## Build

```
cargo test -p slot-store --lib --tests
cargo test -p slot --test systems
cargo build -p slot --release

# Device tree (needs Docker / an aarch64 Debian container and Task)
task dist:device
```
