# This fork

Built from [cloudwithax/unslot](https://github.com/cloudwithax/unslot), itself a fork of [BrandonKowalski/slot](https://github.com/BrandonKowalski/slot).

Extra code was taken from other slot forks rather than rewritten:

| Piece | Source |
|---|---|
| NES, SNES, Genesis, Master System, Sega CD, 32X, Game Gear (and the rest of the shelves) | [cloudwithax/unslot](https://github.com/cloudwithax/unslot) |
| Wi-Fi and Bluetooth | [cloudwithax/unslot](https://github.com/cloudwithax/unslot) |
| HDMI output (sunxi disp2, 1280×720 → 1080p60, audio via `ahubhdmi`) | [nelynes/slotPlus](https://github.com/nelynes/slotPlus) |
| RetroAchievements (`slot-achievements`, vendored rcheevos 12.2.1, banners, offline cache) | [pvaibhav/slot](https://github.com/pvaibhav/slot) (`pvaibhav-improvements`) |

Also inspected, not merged as the base: [MrCTO13/slot-multisystem](https://github.com/MrCTO13/slot-multisystem) (NES/SNES/Genesis/SMS), [nickclyde/slot-plus](https://github.com/nickclyde/slot-plus) (RA + Wi-Fi).

## RetroAchievements

Copy `Config/retroachievements.toml.example` to `Config/retroachievements.toml` and fill in username/password. Softcore only. Console IDs tracked: Mega Drive (1), SNES (3), GB (4), GBA (5), GBC (6), NES (7), Sega CD (9), 32X (10), Master System (11), Game Gear (15).

## HDMI

Switches before EGL starts. A cable change saves and relaunches so the Mali driver can rebuild the window. Closing the lid does nothing while the picture is on the TV.

## Build

```
# Host (macOS/Linux), for tests and a desktop binary
cargo test -p slot-achievements -p slot-gfx --lib
cargo build -p slot --release

# Device tree (needs Docker / an aarch64 Debian container and Task)
task dist:device
```
