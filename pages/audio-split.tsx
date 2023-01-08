import { Button, Paper, Stack, TextField, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import AudioPlayer from "components/AudioPlayer";
import dynamic from "next/dynamic";
import { useEffect, useState } from "react";
import { useDebounce } from "usehooks-ts";

const AmplitudeGraph = dynamic(() => import("components/AmplitudeGraph"), { ssr: false });

export default function AudioSplit() {
    const [splitParams, setSplitParams] = useState({
        threshold: 10000,
        silenceDurSec: 0.5,
        talkDurSec: 1.0,
        extendSec: 0.5,
    });
    const debouncedSplitParams = useDebounce(splitParams, 50);
    
    const [splitRanges, setSplitRanges] = useState<[number, number][]>([]);
    const [significantRanges, setSignificantRanges] = useState<[number, number][]>([[0, -1], [0, -1], [0, -1], [0, -1]]);
    
    useEffect(() => {
        (async () => {
            const _splitRanges = await invoke<[number, number][]>("split_audio", debouncedSplitParams).catch(() => null);
            if (_splitRanges) {
                setSplitRanges(_splitRanges);
            }
            const _significantRanges = await invoke<[number, number][]>("extract_significant_range");
            setSignificantRanges(_significantRanges);
        })();
    }, [debouncedSplitParams]);

    return <>
        <Stack spacing={3} p={2}>
            <Paper elevation={4}>
                <AmplitudeGraph style={{width: "100%", height: 180}} threshold={splitParams.threshold} splitRanges={splitRanges} />
            </Paper>
            <Stack spacing={8} direction="row" justifyContent="center">
                <InputParam label={"しきい値"} defaultValue={10000} step={40} min={1} onChange={val => setSplitParams({ ...splitParams, threshold: val })} />
                <InputParam label={"最短無音時間(秒)"} defaultValue={0.5} step={0.1} min={0.1} onChange={val => setSplitParams({ ...splitParams, silenceDurSec: val })} />
                <InputParam label={"最短分割範囲(秒)"} defaultValue={1.0} step={0.2} min={0.2} onChange={val => setSplitParams({ ...splitParams, talkDurSec: val })} />
                <InputParam label={"拡張時間(秒)"} defaultValue={0.5} step={0.1} min={0} onChange={val => setSplitParams({ ...splitParams, extendSec: val })} />
            </Stack>
            <Stack spacing={0}>
                <Typography variant="caption" align="right">{splitRanges.length}個の分割結果</Typography>
                <AudioPlayer number={1} label="声(最小の平均音量)" segmentRange={significantRanges[0]} />
                <AudioPlayer number={2} label="無音(最大の瞬間音量)" segmentRange={significantRanges[1]} />
                <AudioPlayer number={3} label="声(最長)" segmentRange={significantRanges[2]} />
                <AudioPlayer number={4} label="声(最短)" segmentRange={significantRanges[3]} />
            </Stack>
            <Button variant="outlined">分割完了</Button>
        </Stack>
    </>;
}

function InputParam(
    props: {
        label: string,
        defaultValue: number,
        step: number,
        min: number,
        onChange: (value: number) => void,
    }
) {
    return <>
        <TextField
            type="number"
            label={props.label}
            variant="standard"
            defaultValue={props.defaultValue}
            InputLabelProps={{
                shrink: true,
            }}
            inputProps={{
                step: props.step,
                min: props.min,
            }}
            onChange={(event) => props.onChange(Number(event.target.value))}
        />
    </>;
}