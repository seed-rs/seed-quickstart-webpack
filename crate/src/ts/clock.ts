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