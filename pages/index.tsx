import { css } from '@emotion/react';
import PageSwitch from './PageSwitch';

export default function Home() {
    return <>
        <main css={pageRoot}>
            <PageSwitch />
        </main>
    </>;
}

const pageRoot = css({
    height: "100vh",
});