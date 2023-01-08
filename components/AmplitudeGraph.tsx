import { invoke } from "@tauri-apps/api/tauri";
import { CSSProperties, memo, useEffect, useState } from "react";
import { useDebounce, useElementSize } from "usehooks-ts";
import { Stage, Layer, Line } from "react-konva";

function AmplitudeGraph(
    props: {
        style?: CSSProperties,
        segmentRange?: [number, number],
        threshold?: number,
        splitRanges?: [number, number][],
    }
) {
    const [containerRef, containerSize] = useElementSize();
    const forceUpdateOnResize = useDebounce(containerSize, 200);
    const [renderedSize, setRenderedSize] = useState({
        width: 0,
        height: 0,
    });

    const [amplitudeSamples, setAmplitudeSamples] = useState<number[]>([])
    const [magnification, setMagnification] = useState(1);

    useEffect(() => {
        (async () => {
            const _amplitudeSamples = await invoke<number[]>("extract_amplitude_samples", {
                start: props.segmentRange?.[0] ?? 0,
                end: props.segmentRange?.[1] ?? -1,
                n: containerSize.width,
            }).catch(() => null);
            if (_amplitudeSamples) {
                setRenderedSize(containerSize);
                setAmplitudeSamples(_amplitudeSamples);
                setMagnification(containerSize.height / Math.max(..._amplitudeSamples));
            }
        })();
    }, [forceUpdateOnResize, props.segmentRange]);

    return <>
        <div style={props.style} ref={containerRef}>
            <Stage
                width={containerSize.width}
                height={containerSize.height}
                scale={{
                    x: containerSize.width / renderedSize.width,
                    y: containerSize.height / renderedSize.height,
                }}
            >
                <Layer name="time-label">

                </Layer>
                <Layer name="graph">
                    {amplitudeSamples.map((value, index) => (
                        <Line
                            key={index}
                            x={0.5}
                            y={0.5}
                            stroke="#212121"
                            strokeWidth={1}
                            points={[
                                index, renderedSize.height,
                                index, Math.floor(renderedSize.height - value * magnification)
                            ]}
                        />
                    ))}
                </Layer>
                {props.threshold &&
                    <Layer name="threshold">
                        <Line
                            x={0.5}
                            y={0.5}
                            stroke="#ff1744"
                            strokeWidth={1}
                            points={[
                                0, Math.floor(renderedSize.height - props.threshold * magnification),
                                renderedSize.width, Math.floor(renderedSize.height - props.threshold * magnification),
                            ]}
                        />
                    </Layer>
                }
            </Stage>
        </div>
    </>;
}

export default memo(AmplitudeGraph);