import { invoke } from "@tauri-apps/api";
import { createSignal } from "solid-js";
import "./App.css";
import FileList from "./components/FileList";

export default function App() {
  const [dirPath, setDirPath] = createSignal<string>("");
  const [fileList, setFileList] = createSignal<string[]>([]);

  (async () => {
    const directoryPath = await invoke<string>("open-pick-folder-dialog");
    setFileList(await invoke("get-file-list", {directoryPath}));
  })

  return <>
    <div class="file-list">
      <FileList list={fileList()} />
    </div>
  </>
}