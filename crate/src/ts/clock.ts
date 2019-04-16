
// Example of triggering an event in Typescript and handling in Rust (see `../app.rs`)
// startClock is called from /entries/index.ts

export const startClock = () => {
    dispatchOnClockTickEvent();

    setInterval(() => {
        dispatchOnClockTickEvent();
    }, 1000);
};

const dispatchOnClockTickEvent = () => {
    const event = new CustomEvent('on_clock_tick', { detail: (new Date()).toLocaleTimeString() });
    window.dispatchEvent(event);
}
