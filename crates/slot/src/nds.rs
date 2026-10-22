//! Nintendo DS via standalone Advanced DraStic (H700), not libretro.
//!
//! DraStic is proprietary; the binary is not in this tree. `System/nds/launch.sh`
//! runs it with trngaje's `libadvdrastic.so` and the muOS/MinUI device profiles.

use std::path::Path;
use std::process::Command;

/// Run DraStic for `rom` and wait until it exits.
pub fn run(root: &Path, rom: &Path) -> Result<(), String> {
    let script = root.join("System/nds/launch.sh");
    if !script.is_file() {
        return Err("System/nds/launch.sh is missing".into());
    }
    let mut cmd = Command::new("sh");
    cmd.arg(&script).arg(rom);
    cmd.env("SLOT_ROOT", root);
    if let Ok(target) = std::fs::read_to_string("/etc/baseos-release") {
        if let Some(line) = target.lines().find(|l| l.starts_with("target=")) {
            cmd.env("BASEOS_TARGET", line.trim_start_matches("target=").trim());
        }
    }
    let status = cmd
        .status()
        .map_err(|e| format!("could not start DraStic: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "DraStic exited {}. Put a 64-bit drastic binary in System/nds/ (see Logs/NDS.txt).",
            status.code().unwrap_or(-1)
        ))
    }
}
