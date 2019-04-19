import { createCustomEvent, customEventIdEnum } from './seed_helpers'

// Example of triggering an event in Typescript and handling in Rust (see `../app.rs`)
// startClock is called from /entries/index.ts

export const startClock = () => {
    dispatchOnClockTickEvent();

    setInterval(() => {
        dispatchOnClockTickEvent();
    }, 1000);
};

const dispatchOnClockTickEvent = () => {
    const time = (new Date()).toLocaleTimeString();
    const event = createCustomEvent(customEventIdEnum().OnClockTick, time)
    window.dispatchEvent(event);
}
