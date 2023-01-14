import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/router";

type FileType = "open_file" | "open_project";

export default function useSelectFile() {
    const router = useRouter();

    const selectFile = async (type: FileType, pathname: string) => {
        try {
            const path = await invoke<string>(type);
            const audioUrl = convertFileSrc(path);
            router.push({
                pathname,
                query: { audio: audioUrl },
            });
        } catch {
            return;
        }
    };
    return selectFile;
}