import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

type UseSignificantRangesResult = [[number, number][], () => Promise<void>];

export default function useSignificantRanges(): UseSignificantRangesResult {
    const [significantRanges, setSignificantRanges] = useState<[number, number][]>([]);

    const updateSignificantRanges = async () => {
        try {
            const _significantRanges = await invoke<[number, number][]>("extract_significant_range");
            setSignificantRanges(_significantRanges);
        } catch {
            return;
        }
    };
    return [significantRanges, updateSignificantRanges];
}