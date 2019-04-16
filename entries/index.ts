import "../css/styles.css";
import { startClock } from  "../crate/src/ts/clock";

(async () => {
    // We have to load wasm async
    // NOTE: files in crate/pkg/ will be created on first build
    (await import('../crate/pkg/appname')).run();
    startClock();
})()
