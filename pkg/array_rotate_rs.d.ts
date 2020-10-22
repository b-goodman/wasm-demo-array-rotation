/* tslint:disable */
/* eslint-disable */
/**
* @param {any} vec_input
* @returns {any}
*/
export function rotate_left_2d(vec_input: any): any;
/**
* @param {any} vec_input
* @param {number} steps
* @returns {any}
*/
export function rotate_right(vec_input: any, steps: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly rotate_left_2d: (a: number) => number;
  readonly rotate_right: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        