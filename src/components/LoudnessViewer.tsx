import { createEffect, onMount } from "solid-js";

export default function LoudnessViewer(
    props: {
        samples: number[],
    }
) {
    let canvas: HTMLCanvasElement | undefined;
    createEffect(() => {
        const ctx = canvas!.getContext("2d", { alpha: false })!;

        ctx.canvas.width = 3840;
        ctx.canvas.height = 480;
        
        ctx.fillStyle = "white";
        ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

        const max = Math.max(...props.samples);
        
        ctx.beginPath();
        props.samples.forEach((sample, index) => {
            ctx.lineWidth = 1;
            ctx.moveTo(index, ctx.canvas.height);
            ctx.lineTo(index, ctx.canvas.height - sample * (ctx.canvas.height / max));
            ctx.stroke();
        });
    });

    return <>
        <canvas id="loudness-viewer" ref={canvas} style={{width: "100%"}} />
    </>
}