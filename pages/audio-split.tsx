import { Button, Paper, Stack, TextField, Typography } from "@mui/material";
import AudioPlayer from "components/AudioPlayer";
import useSignificantRanges from "hooks/useSignificantRanges";
import useSplitRanges from "hooks/useSplitRanges";
import dynamic from "next/dynamic";
import { useState } from "react";
import { useDebounce } from "react-use";
import { SplitParams } from "types/SplitParams";

const AmplitudeGraph = dynamic(() => import("components/AmplitudeGraph"), { ssr: false });

export default function AudioSplit() {
    const [splitParams, setSplitParams] = useState<SplitParams>({
        threshold: 10000,
        silenceDurSec: 0.5,
        talkDurSec: 1.0,
        extendSec: 0.5,
    });
    const [splitRanges, setSplitRanges] = useSplitRanges();
    const [significantRanges, updateSignificantRanges] = useSignificantRanges();

    useDebounce(() => {
        setSplitRanges(splitParams);
        updateSignificantRanges();
    }, 100, [splitParams]);

    return <>
        <Stack spacing={3} p={2}>
            <Paper elevation={4}>
                <AmplitudeGraph style={{width: "100%", height: 180}} threshold={splitParams.threshold} splitRanges={splitRanges} />
            </Paper>
            <Stack spacing={8} direction="row" justifyContent="center">
                <InputParam label={"しきい値"} defaultValue={10000} step={40} min={1} onChange={val => setSplitParams({ ...splitParams, threshold: val })} />
                <InputParam label={"最短無音時間(秒)"} defaultValue={0.5} step={0.1} min={0.1} onChange={val => setSplitParams({ ...splitParams, silenceDurSec: val })} />
                <InputParam label={"最短分割範囲(秒)"} defaultValue={0.2} step={0.1} min={0.05} onChange={val => setSplitParams({ ...splitParams, talkDurSec: val })} />
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