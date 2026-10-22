#!/bin/sh
# Nintendo DS via Advanced DraStic on Allwinner H700 (RG34XX, RG35XX*, RG SP).
# DraStic itself is proprietary: put the 64-bit binary in this folder as `drastic`
# (or `drastic_v2522`). libadvdrastic.so is trngaje's H700 hook.
# Device profiles follow muOS / minui-nintendo-ds-pak.
set -eu

DIR="$(CDPATH= cd -- "$(dirname "$0")" && pwd)"
ROM="${1:-}"
LOG="${SLOT_ROOT:-/mnt/sdcard}/Logs/NDS.txt"
mkdir -p "$(dirname "$LOG")"
exec >>"$LOG" 2>&1
echo "$(date -u +%Y-%m-%dT%H:%M:%SZ) launch $ROM"

bin=""
for c in "$DIR/drastic" "$DIR/drastic_v2522" "$DIR/drastic64"; do
	if [ -x "$c" ]; then
		bin="$c"
		break
	fi
done
if [ -z "$bin" ]; then
	echo "no DraStic binary in $DIR (need drastic or drastic_v2522)"
	exit 1
fi
if [ -z "$ROM" ] || [ ! -e "$ROM" ]; then
	echo "no rom: $ROM"
	exit 1
fi

target="$(tr '[:upper:]' '[:lower:]' < /etc/baseos-release 2>/dev/null | sed -n 's/^target=//p' | head -n 1)"
[ -n "$target" ] || target="${BASEOS_TARGET:-}"
[ -n "$target" ] || target="${SLOT_DEVICE:-}"

# minui-nintendo-ds-pak / Advanced DraStic H700 profiles
profile="rg35xx-sp"
case "$target" in
	rg28xx) profile="rg28xx" ;;
	rg35xxsp) profile="rg35xx-sp" ;;
	rg40xxv) profile="rg40xx-v" ;;
	rgcubexx) profile="rg-cubexx" ;;
	rg40xxh | rg35xxh | rg35xxpro | rg34xxsp) profile="rg40xx-h" ;;
	rg34xx | rg35xxplus | rg35xx | rgsp) profile="rg35xx-sp" ;;
esac
echo "target=$target profile=$profile"

mkdir -p "$DIR/config" "$DIR/backup" "$DIR/savestates" "$DIR/cheats"
if [ -f "$DIR/devices/$profile/config/drastic.cfg" ]; then
	cp -f "$DIR/devices/$profile/config/drastic.cfg" "$DIR/config/drastic.cfg"
fi
if [ -f "$DIR/devices/$profile/config/drastic.cf2" ]; then
	cp -f "$DIR/devices/$profile/config/drastic.cf2" "$DIR/config/drastic.cf2"
fi

# ~1.6 GHz like the MinUI pak; fall back to the CPU's max.
policy=""
for d in /sys/devices/system/cpu/cpufreq/policy0 /sys/devices/system/cpu/cpu0/cpufreq; do
	if [ -r "$d/scaling_available_frequencies" ]; then
		policy="$d"
		break
	fi
done
if [ -n "$policy" ]; then
	echo ondemand >"$policy/scaling_governor" 2>/dev/null || echo performance >"$policy/scaling_governor" || true
	awk -v limit=1608000 '{
		for (i = 1; i <= NF; i++) {
			if (max == "" || $i > max) max = $i
			if ($i <= limit && (val == "" || $i > val)) val = $i
		}
	} END { print (val == "" ? max : val) }' "$policy/scaling_available_frequencies" >"$policy/scaling_max_freq" || true
fi

case "$(echo "$ROM" | tr '[:upper:]' '[:lower:]')" in
	*.zip)
		tmp="$(mktemp -d /tmp/nds_rom_XXXXXX)"
		unzip -o -q "$ROM" -d "$tmp" || true
		nds="$(find "$tmp" -iname '*.nds' | head -n 1)"
		[ -n "$nds" ] && ROM="$nds"
		;;
esac

export HOME="$DIR"
export LD_LIBRARY_PATH="$DIR/libs:${LD_LIBRARY_PATH:-}"
unset LD_PRELOAD
if [ -f "$DIR/libs/libadvdrastic.so" ]; then
	export LD_PRELOAD="$DIR/libs/libadvdrastic.so"
fi

cd "$DIR"
echo "exec $bin"
"$bin" "$ROM"
code=$?
echo "drastic exit $code"
exit "$code"
