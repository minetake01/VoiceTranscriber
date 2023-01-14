import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { SplitParams } from "types/SplitParams";

type UseSplitRangesResult = [[number, number][], (splitParams: SplitParams) => Promise<void>];

export default function useSplitRanges(): UseSplitRangesResult {
    const [splitRanges, setSplitRanges] = useState<[number, number][]>([]);

    const updateSplitRanges = async (splitParams: SplitParams) => {
        try {
            const _splitRanges = await invoke<[number, number][]>("split_audio", splitParams);
            setSplitRanges(_splitRanges);
        } catch {
            return;
        }
    };
    return [splitRanges, updateSplitRanges];
}