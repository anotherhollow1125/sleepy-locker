import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import "./App.css";
import { getVersion } from "@tauri-apps/api/app";
import SleepPreventSwitch from "@/components/SleepPreventSwitch";
import StartUpSwitch from "@/components/StartUpSwitch";

function App() {
  const [version, setVersion] = useState("");

  useEffect(() => {
    getVersion().then((v) => {
      setVersion(v);
    });
  }, []);

  return (
    <div className="container">
      <Grid container spacing={2} sx={{ px: 2, textAlign: "left" }}>
        <Grid item xs={8}>
          <h4>Toggle Sleep Prevent</h4>
        </Grid>
        <Grid
          item
          xs={4}
          sx={{
            display: "flex",
            flexDirection: "row-reverse",
            alignItems: "center",
          }}
        >
          <SleepPreventSwitch />
        </Grid>
      </Grid>

      <Grid container spacing={2} sx={{ px: 2, textAlign: "left" }}>
        <Grid item xs={8}>
          <h4>Start Up</h4>
        </Grid>
        <Grid
          item
          xs={4}
          sx={{
            display: "flex",
            flexDirection: "row-reverse",
            alignItems: "center",
          }}
        >
          <StartUpSwitch />
        </Grid>
      </Grid>

      <Grid
        container
        spacing={2}
        sx={{ px: 2, mt: 2, textAlign: "right", alignItems: "center" }}
      >
        <Grid item xs={12} sx={{ textAlign: "left" }}>
          <h4>{"Sleepy Locker Ver. " + version}</h4>
        </Grid>
      </Grid>
    </div>
  );
}

export default App;
