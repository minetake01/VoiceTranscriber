import { invoke } from "@tauri-apps/api/tauri";
import { Button, Grid, Stack } from "@mui/material";
import { useState } from "react";
import { Pages } from "../PageSwitch";

export default function FileSelectPage(props: {setPage: (pages: Pages) => void}) {
    const [processing, setProcessing] = useState(false);

    const openFile = () => {
        invoke("select_file").then(async () => {
            setProcessing(true);
            await invoke("decode");
            props.setPage(Pages.AudioSplit);
            setProcessing(false);
        }).catch(() => { return; });
    };
    
    return <>
        <Grid container justifyContent="center" alignItems="center" height="100%">
            <Stack spacing={1} direction="row">
                <Button disabled={processing} variant="contained" onClick={openFile}>
                    ファイルを開く
                </Button>
                <Button disabled={processing} variant="outlined">
                    プロジェクトを開く
                </Button>
            </Stack>
        </Grid>
    </>;
}
