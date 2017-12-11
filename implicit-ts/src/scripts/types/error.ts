export default interface Error {
    message: string,
    line_num: number,
    col_num: number,
}

// monaco
export interface ErrorStructure {
    start: number,
    length: number,
    messageText: string | ErrorStructure,
}
