import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { createContext } from 'react';

export enum Pages {
    FileSelect,
    AudioSplit,
    Labeling,
    Export,
}

export const PagesContext = createContext<Pages>(Pages.FileSelect);

export default function App({ Component, pageProps }: AppProps) {
    return <>
        <PagesContext.Provider value={Pages.FileSelect}>
            <Component {...pageProps} />
        </PagesContext.Provider>
    </>;
}
