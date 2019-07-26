/**
 * Fire event that can be handled by Seed application
 *
 * @param {string} msgName Msg variant name, e.g. Tick(String) will be "Tick"
 * @param {*|undefined} data Serialized Msg variant data
 */
export const triggerUpdate = (msgName: string, msgData: any = undefined) => {
  const event = new CustomEvent("triggerupdate", {
    detail: msgData === undefined ? msgName : { [msgName]: msgData }
  });
  window.dispatchEvent(event);
};
