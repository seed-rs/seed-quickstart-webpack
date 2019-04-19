// see /entries/index.ts
export type CustomEventId = typeof window.rustModule.CustomEventId.NoOp
// it has to be "lazy" because rustModule is loaded async in index.ts
export const customEventIdEnum = () => window.rustModule.CustomEventId

export const createCustomEvent = (customEventId: CustomEventId, data: any): CustomEvent => {
    return new CustomEvent(customEventId.toString(), { detail: data })
}

// Seed doesn't use RAF in a native render loop at the time of writing
export const callRequestAnimationFrame = (): void => {
    requestAnimationFrame(() => {
        const event = createCustomEvent(customEventIdEnum().OnRequestAnimationFrame, undefined);
        window.dispatchEvent(event);
    })
}
