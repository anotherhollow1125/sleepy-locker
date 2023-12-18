import Switch from "@mui/material/Switch";
import { enable, disable, isEnabled } from "tauri-plugin-autostart-api";
import { useEffect, useState, ChangeEvent } from "react";

export default function StartUpSwitch() {
  const [checked, setChecked] = useState(false);
  const handleChange = async (event: ChangeEvent<HTMLInputElement>) => {
    setChecked(event.target.checked);
    if (event.target.checked) {
      await enable();
    } else {
      await disable();
    }
  };

  useEffect(() => {
    let cancel = false;

    (async () => {
      let enabled = await isEnabled();
      if (!cancel) {
        setChecked(enabled);
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
