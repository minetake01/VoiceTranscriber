import { PlayCircle, StopCircle } from "@mui/icons-material";
import { IconButton, Stack, Typography } from "@mui/material";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import dynamic from "next/dynamic";
import { memo } from "react";
import { useAsync, useAudio } from "react-use";

const AmplitudeGraph = dynamic(() => import("components/AmplitudeGraph"), { ssr: false });

function AudioPlayer(
    props: {
        number: number,
        label: string,
        segmentRange: [number, number],
        processing?: boolean,
    }
) {
    const audio_segment = useAsync(async () => {
        const path = await invoke<string>("encode_partial", {start: props.segmentRange[0], end: props.segmentRange[1]});
        return convertFileSrc(path);
    }, [props.segmentRange]);
    
    const [audio, state, controls, _ref] = useAudio({
        src: audio_segment.value || ""
    });
    
    return <>
        {audio}
        <Stack direction="row" alignItems="center">
            <Typography variant="caption">{props.number}</Typography>
            <IconButton onClick={() => {
                state.playing ? controls.pause() : controls.play();
            }}>
                {state.playing ? <StopCircle /> : <PlayCircle />}
            </IconButton>
            <Stack flexGrow={1} minWidth={0} spacing={1}>
                <Typography variant="caption">{props.label}</Typography>
                <AmplitudeGraph style={{width: "100%", height: 25}} segmentRange={props.segmentRange} />
            </Stack>
        </Stack>
    </>;
}

export default memo(AudioPlayer);