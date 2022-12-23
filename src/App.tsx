import { createSignal, Match, Switch } from "solid-js";
import Notification from "./components/Notification";
import AudioSplit from "./pages/AudioSplit";
import Export from "./pages/Export";
import FileSelect from "./pages/FileSelect";
import Labeling from "./pages/Labeling";

export enum Pages {
    FileSelect,
    AudioSplit,
    Labeling,
    Export,
}

export const [page, setPage] = createSignal<Pages>(Pages.FileSelect);
export const [notices, setNotices] = createSignal<string[]>([]);

export default function App() {
    return <>
        <Switch>
            <Match when={page() === Pages.FileSelect}>
                <FileSelect />
            </Match>
            <Match when={page() === Pages.AudioSplit}>
                <AudioSplit />
            </Match>
            <Match when={page() === Pages.Labeling}>
                <Labeling />
            </Match>
            <Match when={page() === Pages.Export}>
                <Export />
            </Match>
        </Switch>
        <Notification />
    </>
}