import { css } from "@emotion/react";
import { memo, useEffect } from "react";

function LoudnessViewer(
    props: {
        height: number,
        samples: number[],
        threshold?: number,
    }
) {
    const canvasHeight = props.height;
    const canvasWeight = props.samples.length;

    const magnification = canvasHeight / Math.max(...props.samples);

    //ラウドネスグラフの描画
    useEffect(() => {
        const canvas = document.getElementById("loudness") as HTMLCanvasElement;
        const ctx = canvas.getContext("2d", { alpha: false })!;

        //白埋め
        ctx.fillStyle = "white";
        ctx.fillRect(0, 0, canvasWeight, canvasHeight);

        //グラフ描画
        ctx.strokeStyle = "black";
        for (let i = 0; i < canvasWeight; i++) {
            ctx.beginPath();
            ctx.moveTo(i, canvasHeight);
            ctx.lineTo(i, Math.floor(canvasHeight - props.samples[i] * magnification));
            ctx.closePath();
            ctx.stroke();
        }
    }, [props.samples]);

    //しきい値の線を描画
    useEffect(() => {
        if (props.threshold) {
            const canvas = document.getElementById("threshold") as HTMLCanvasElement;
            const ctx = canvas.getContext("2d")!;

            const lineY = Math.floor(canvasHeight - props.threshold * magnification);
    
            ctx.clearRect(0, 0, canvasWeight, canvasHeight);
            ctx.strokeStyle = "red";
            ctx.beginPath();
            ctx.moveTo(0, lineY);
            ctx.lineTo(canvasWeight, lineY);
            ctx.closePath();
            ctx.stroke();
        }
    }, [props.samples, props.threshold]);

    const styles = {
        canvasContainer: css({
            position: "relative",
            width: "100%",
            height: props.height + 4,
        }),
        canvasLayer: css({
            position: "absolute",
            width: "100%",
            height: props.height,
        }),
    };

    return <div css={styles.canvasContainer}>
        <canvas css={styles.canvasLayer} height={canvasHeight} width={canvasWeight} id="loudness" />
        <canvas css={styles.canvasLayer} height={canvasHeight} width={canvasWeight} id="threshold" />
    </div>;
}

export default memo(LoudnessViewer);