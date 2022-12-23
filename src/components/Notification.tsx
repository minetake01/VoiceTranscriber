import { Box, LinearProgress, Paper, Slide, Stack, Typography } from "@suid/material";
import { Info } from "@suid/icons-material"
import { For } from "solid-js";
import { notices } from "../App";

export default function Notification() {
    //TODO: エラーの通知を実装する
    return <>
        <Slide direction="left" in={!!notices().length}>
            <Paper sx={{ m: 1, width: 240, position: "absolute", right: 8, bottom: 8 }} elevation={2}>
                <For each={notices()}>{(item) => 
                    <Box sx={{ position: "relative" }}>
                        <Stack direction="row" spacing={1} alignItems="center" sx={{ padding: "12px" }}>
                            <Info color="info" />
                            <Typography variant="body2" noWrap>{item}</Typography>
                        </Stack>
                        <Box sx={{ position: "absolute", width: "100%", bottom: 0 }}>
                            <LinearProgress />
                        </Box>
                    </Box>
                }</For>
            </Paper>
        </Slide>
    </>;
}