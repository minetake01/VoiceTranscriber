import { PlayCircle, StopCircle } from "@mui/icons-material";
import { IconButton, Stack, Typography } from "@mui/material";
import dynamic from "next/dynamic";
import { memo, useState } from "react";

const AmplitudeGraph = dynamic(() => import("components/AmplitudeGraph"), { ssr: false });

function AudioPlayer(
    props: {
        number: number,
        label: string,
        segmentRange: [number, number],
        processing?: boolean,
    }
) {
    const [playing, setPlaying] = useState(false);
    
    return <>
        <Stack direction="row" alignItems="center">
            <Typography variant="caption">{props.number}</Typography>
            <IconButton onClick={() => setPlaying(!playing)}>
                {playing ? <StopCircle /> : <PlayCircle />}
            </IconButton>
            <Stack flexGrow={1} minWidth={0} spacing={1}>
                <Typography variant="caption">{props.label}</Typography>
                <AmplitudeGraph style={{width: "100%", height: 20}} segmentRange={props.segmentRange} />
            </Stack>
        </Stack>
    </>;
}

export default memo(AudioPlayer);