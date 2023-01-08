import { Button, Grid, Stack } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/router";

export default function FileSelect() {
    const router = useRouter();

    const openFile = async () => {
        invoke("open_file").then(() => {
            router.push("/audio-split");
        }).catch(() => {
            return;
        });
    }
    
    const openProject = async () => {
        invoke("open_project").then(() => {
            router.push("/labeling");
        }).catch(() => {
            return;
        });
    }
    
    return <>
        <Grid container justifyContent="center" alignItems="center" height="100%">
            <Stack spacing={1} direction="row">
                <Button variant="contained" onClick={openFile}>
                    ファイルを開く
                </Button>
                <Button variant="outlined" onClick={openProject}>
                    プロジェクトを開く
                </Button>
            </Stack>
        </Grid>
    </>;
}
