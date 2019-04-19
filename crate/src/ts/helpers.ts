// Example of function that is called from Rust (see ../ts_apis.rs)

export const get_random_number = (min: number, max: number): number => {
    return Math.floor(Math.random() * (max - min + 1) ) + min;
}
