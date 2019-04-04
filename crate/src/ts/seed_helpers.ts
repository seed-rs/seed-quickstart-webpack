
export const callRequestAnimationFrame = (): void => {
    requestAnimationFrame(() => {
        const event = new CustomEvent('on_request_animation_frame');
        window.dispatchEvent(event);
    })
}