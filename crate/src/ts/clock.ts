import { triggerUpdate } from "./seed_helpers";

// Example of triggering an event in Typescript and handling in Rust (see `../app.rs`)
// startClock is called from /entries/index.ts

export const startClock = () => {
  const triggerTickEvent = () => {
    triggerUpdate("OnClockTick", new Date().toLocaleTimeString());
  };
  triggerTickEvent();

  setInterval(() => {
    triggerTickEvent();
  }, 1000);
};
