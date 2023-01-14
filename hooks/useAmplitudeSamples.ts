import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

type UseAmplitudeSamplesResult = [number[], (start: number, end: number, n: number) => Promise<void>];

export default function useAmplitudeSamples(): UseAmplitudeSamplesResult {
    const [amplitudeSamples, setAmplitudeSamples] = useState<number[]>([]);

    const updateAmplitudeSamples = async (
        start: number,
        end: number,
        n: number,
    ) => {
        try {
            const _amplitudeSamples = await invoke<number[]>("extract_amplitude_samples", { start, end, n });
            setAmplitudeSamples(_amplitudeSamples);
        } catch {
            return;
        }
    };
    return [amplitudeSamples, updateAmplitudeSamples];
}