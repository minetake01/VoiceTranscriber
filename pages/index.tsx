import { Button, Grid, Stack } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/router";
import { useState } from "react";

export default function FileSelect() {
    const router = useRouter();
    const [processing, setProcessing] = useState(false);

    return <>
        <Grid container justifyContent="center" alignItems="center" height="100%">
            <Stack spacing={1} direction="row">
                <Button variant="contained" disabled={processing} onClick={async () => {
                    setProcessing(true);
                    invoke("open_file").then(() => {
                        router.push("/audio-split");
                    }).finally(() => {
                        setProcessing(false);
                    });
                }}>
                    ファイルを開く
                </Button>
                <Button variant="outlined" disabled={processing} onClick={async () => {
                    setProcessing(true);
                    invoke("open_project").then(() => {
                        router.push("/labeling");
                    }).finally(() => {
                        setProcessing(false);
                    });
                }}>
                    プロジェクトを開く
                </Button>
            </Stack>
        </Grid>
    </>;
}
