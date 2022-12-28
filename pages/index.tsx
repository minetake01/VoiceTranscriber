import Head from 'next/head';
import PageSwitch from './PageSwitch';

export default function Home() {
    return <>
        <Head>
            <title>Train data creator</title>
            <meta name="description" content="Elite35P-ServerのEliteVoiceProjectで使用する、学習用音声データセットを効率的に制作する為のGUIアプリ。" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link rel="icon" href="/favicon.ico" />
        </Head>
        <main>
            <PageSwitch />
        </main>
    </>;
}
