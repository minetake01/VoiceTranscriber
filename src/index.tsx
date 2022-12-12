/* @refresh reload */
import { render } from "solid-js/web";

import "./style.css";
import "../node_modules/hiq/dist/hiq.min.css";
import App from "./App";

render(() => <App />, document.getElementById("root") as HTMLElement);
