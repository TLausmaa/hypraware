import subprocess
import json
import sys

all_monitors = subprocess.check_output(["hyprctl", "monitors", "all", "-j"]).decode("utf-8")
enabled_monitors = subprocess.check_output(["hyprctl", "monitors", "-j"]).decode("utf-8")

all_monitors_json = json.loads(all_monitors)
enabled_monitors_json = json.loads(enabled_monitors)
all_count = len(all_monitors_json)
enabled_count = len(enabled_monitors_json)

if all_count == 1 and enabled_count == 1:
    # enable all screens if only one is connected
    subprocess.run(["hyprctl", "keyword", "monitor", ",preferred,auto,auto"])
elif all_count > 1 and enabled_count > 1 and all_count == enabled_count:
    # disable the first (built-in) monitor if more than two are connected
    subprocess.run(["hyprctl", "keyword", "monitor", all_monitors_json[0]["name"] + ",disable"])
elif all_count > enabled_count:
    pass
    # we have already disabled some monitors, do nothing

# monitor=,preferred,auto,auto
# monitor= eDP-1, disable # Disable built-in laptop screen

