import Switch from "@mui/material/Switch";
import { useEffect, useState, ChangeEvent } from "react";
import { invoke } from "@tauri-apps/api";

export default function SleepPreventSwitch() {
  const [checked, setChecked] = useState(false);
  const handleChange = async (event: ChangeEvent<HTMLInputElement>) => {
    setChecked(event.target.checked);
    const new_state = event.target.checked;
    await invoke<boolean>("set_sleep_prevent_enabled", { enabled: new_state });
    setChecked(new_state);
  };

  useEffect(() => {
    let cancel = false;
    (async () => {
      const res = await invoke<boolean>("get_sleep_prevent_enabled");
      if (!cancel) {
        setChecked(res);
      }
    })();

    return () => {
      cancel = true;
    };
  }, []);

  return (
    <Switch
      checked={checked}
      onChange={handleChange}
      inputProps={{ "aria-label": "controlled" }}
    />
  );
}
