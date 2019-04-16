
// Seed doesn't use RAF in a native render loop at the time of writing
export const callRequestAnimationFrame = (): void => {
    requestAnimationFrame(() => {
        const event = new CustomEvent('on_request_animation_frame');
        window.dispatchEvent(event);
    })
}
