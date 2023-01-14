import useAmplitudeSamples from "hooks/useAmplitudeSamples";
import { CSSProperties, memo, useEffect, useState } from "react";
import { Stage, Layer, Line } from "react-konva";
import { useDebounce, useMeasure } from "react-use";

function AmplitudeGraph(
    props: {
        style?: CSSProperties,
        segmentRange?: [number, number],
        threshold?: number,
        splitRanges?: [number, number][],
    }
) {
    const [containerRef, containerSize] = useMeasure<HTMLDivElement>();
    const [amplitudeSamples, updateAmplitudeSamples] = useAmplitudeSamples();

    useDebounce(() => {
        updateAmplitudeSamples(
            props.segmentRange?.[0] ?? 0,
            props.segmentRange?.[1] ?? -1,
            containerSize.width,
        );
    }, 300, [containerSize, props.segmentRange]);

    const [renderedSize, setRenderedSize] = useState({ width: 0, height: 0 });
    const [magnification, setMagnification] = useState(1);

    useEffect(() => {
        setRenderedSize(containerSize);
        setMagnification(containerSize.height / Math.max(...amplitudeSamples));
    }, [amplitudeSamples, props.segmentRange]);

    return <>
        <div style={props.style} ref={containerRef}>
            <Stage
                width={Math.floor(containerSize.width)}
                height={Math.floor(containerSize.height)}
                scale={{
                    x: containerSize.width / renderedSize.width,
                    y: containerSize.height / renderedSize.height,
                }}
            >
                <Layer>
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
                            perfectDrawEnabled={false}
                            shadowForStrokeEnabled={false}
                            hitStrokeWidth={0}
                        />
                    ))}
                </Layer>
                {props.threshold &&
                    <Layer>
                        <Line
                            x={0.5}
                            y={0.5}
                            stroke="#ff1744"
                            strokeWidth={1}
                            points={[
                                0, Math.floor(renderedSize.height - props.threshold * magnification),
                                renderedSize.width, Math.floor(renderedSize.height - props.threshold * magnification),
                            ]}
                            perfectDrawEnabled={false}
                            shadowForStrokeEnabled={false}
                            hitStrokeWidth={0}
                        />
                    </Layer>
                }
            </Stage>
        </div>
    </>;
}

export default memo(AmplitudeGraph);