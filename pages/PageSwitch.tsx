import { useContext } from "react";
import AudioSplitPage from "./app/AudioSplit";
import ExportPage from "./app/Export";
import FileSelectPage from "./app/FileSelect";
import LabelingPage from "./app/Labeling";
import { Pages, PagesContext } from "./_app";

export default function PageSwitch() {
    const Page = () => {
        switch (useContext(PagesContext)) {
            case Pages.FileSelect:
                return <FileSelectPage />
            case Pages.AudioSplit:
                return <AudioSplitPage />
            case Pages.Labeling:
                return <LabelingPage />
            case Pages.Export:
                return <ExportPage />
        }
    };

    return <Page />;
}
