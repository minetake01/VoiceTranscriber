import { PlayCircle, StopCircle } from "@mui/icons-material";
import { IconButton, Stack, Typography } from "@mui/material";
import dynamic from "next/dynamic";
import { useRouter } from "next/router";
import { memo } from "react";
import { useAudio } from "react-use";

const AmplitudeGraph = dynamic(() => import("components/AmplitudeGraph"), { ssr: false });

function AudioPlayer(
    props: {
        number: number,
        label: string,
        segmentRange: [number, number],
        processing?: boolean,
    }
) {
    const router = useRouter();
    const audioUrl = router.query.audio as string;
    const [audio, state, controls] = useAudio({ src: audioUrl });
    
    return <>
        <Stack direction="row" alignItems="center">
            {audio}
            <Typography variant="caption">{props.number}</Typography>
            <IconButton onClick={() => {
                state.paused ? controls.play() : controls.pause()
            }}>
                {state.playing ? <StopCircle /> : <PlayCircle />}
            </IconButton>
            <Stack flexGrow={1} minWidth={0} spacing={1}>
                <Typography variant="caption">{props.label}</Typography>
                <AmplitudeGraph style={{width: "100%", height: 20}} segmentRange={props.segmentRange} />
            </Stack>
        </Stack>
    </>;
}

export default memo(AudioPlayer);