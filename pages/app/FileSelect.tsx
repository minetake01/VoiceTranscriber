import { invoke } from "@tauri-apps/api/tauri";
import { css } from "@emotion/react";
import { Button, Grid, Stack } from "@mui/material";
import { useContext, useState } from "react";
import { Pages, SetPagesContext } from "../_app";

export default function FileSelectPage() {
    const [processing, setProcessing] = useState(false);
    const setPages = useContext(SetPagesContext);

    const openFile = () => {
        invoke("select_file").then(async () => {
            setProcessing(true);
            await invoke("decode");
            setPages(Pages.AudioSplit);
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
