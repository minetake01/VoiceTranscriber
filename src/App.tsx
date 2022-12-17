import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { createSignal, Show } from "solid-js";
import "./App.css";
import LoudnessViewer from "./components/LoudnessViewer";

export default function App() {
    const [sampleRate, setSampleRate] = createSignal<number>();
    const [samples, setSamples] = createSignal<number[]>([]);

    listen<number>("decoded", event => {
        setSampleRate(event.payload);

        (async () => {
            const samples = await invoke<number[]>("samples_extraction", {
                start: 0,
                end: -1,
                n: 3840,
            });
            setSamples(samples);
        })();
    });

    return <>
        <Show when={sampleRate()} fallback={<div>ファイルを選択してください。</div>}>
            <LoudnessViewer samples={samples()} />
        </Show>
    </>
}