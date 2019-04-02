export const startClock = () => {
    dispatchOnClockTickEvent();

    setInterval(() => {
        dispatchOnClockTickEvent();
    }, 1000);
};

const dispatchOnClockTickEvent = () => {
    const event = new CustomEvent('onclocktick', { detail: (new Date()).toLocaleTimeString() });
    window.dispatchEvent(event);
}