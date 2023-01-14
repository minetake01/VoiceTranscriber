import { Button, Grid, Stack } from "@mui/material";
import useSelectFile from "hooks/useSelectFile";

export default function FileSelect() {
    const selectFile = useSelectFile();
    
    return <>
        <Grid container justifyContent="center" alignItems="center" height="100%">
            <Stack spacing={1} direction="row">
                <Button variant="contained" onClick={() => selectFile("open_file", "/audio-split")}>
                    ファイルを開く
                </Button>
                <Button variant="outlined" onClick={() => selectFile("open_project", "/labeling")}>
                    プロジェクトを開く
                </Button>
            </Stack>
        </Grid>
    </>;
}
