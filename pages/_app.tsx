import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { createContext, Dispatch, SetStateAction, useState } from 'react';

export enum Pages {
    FileSelect,
    AudioSplit,
    Labeling,
    Export,
}

export const PagesContext = createContext<Pages>(Pages.FileSelect);
export const SetPagesContext = createContext<Dispatch<SetStateAction<Pages>>>(() => undefined);

export default function App({ Component, pageProps }: AppProps) {
    const [page, setPage] = useState<Pages>(Pages.FileSelect);

    return <>
        <PagesContext.Provider value={page}>
        <SetPagesContext.Provider value={setPage}>
            <Component {...pageProps} />
        </SetPagesContext.Provider>
        </PagesContext.Provider>
    </>;
}
