import { useState } from "react";
import AudioSplitPage from "./app/AudioSplit";
import ExportPage from "./app/Export";
import FileSelectPage from "./app/FileSelect";
import LabelingPage from "./app/Labeling";

export enum Pages {
    FileSelect,
    AudioSplit,
    Labeling,
    Export,
}

export default function PageSwitch() {
    const [page, setPage] = useState(Pages.FileSelect);
    
    const Page = () => {
        switch (page) {
            case Pages.FileSelect:
                return <FileSelectPage setPage={(page) => setPage(page)} />
            case Pages.AudioSplit:
                return <AudioSplitPage setPage={(page) => setPage(page)} />
            case Pages.Labeling:
                return <LabelingPage setPage={(page) => setPage(page)} />
            case Pages.Export:
                return <ExportPage setPage={(page) => setPage(page)} />
        }
    };

    return <Page />;
}
