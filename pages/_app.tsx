import 'styles/globals.css';
import type { AppProps } from 'next/app';
import Head from 'next/head';
import { RecoilRoot } from 'recoil';

export default function App({ Component, pageProps }: AppProps) {
    return <>
        <Head>
            <title>VoiceTranscriber</title>
            <meta name="description" content="Elite35P-ServerのEliteVoiceProjectで使用する、学習用音声データセットを効率的に制作する為のGUIアプリ。" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
        </Head>
        <RecoilRoot>
            <Component {...pageProps} />
        </RecoilRoot>
    </>;
}