import { Button, Grid, Stack } from "@suid/material";
import { invoke } from "@tauri-apps/api";
import { createSignal } from "solid-js";
import { notices, Pages, setNotices, setPage } from "../App";

export default function FileSelect() {
    const [processing, setProcessing] = createSignal(false);

    async function openFile() {
        const path = await invoke<string>("select_file");
        if (path) {
            setNotices([...notices(), "デコード処理中…"]);
            setProcessing(true);
            await invoke("decode", { path: path });
            setNotices([...notices()].filter(item => item !== "デコード処理中…"));
            setProcessing(false);
            setPage(Pages.AudioSplit);
        }
    }

    return <>
        <Grid container alignItems="center" justifyContent="center">
            <Stack spacing={1} direction="row">
                <Button disabled={processing()} variant="contained" onClick={() => openFile()}>
                    ファイルを開く
                </Button>
                <Button disabled={processing()} variant="outlined" component="label">
                    プロジェクトを開く
                </Button>
            </Stack>
        </Grid>
    </>;
}