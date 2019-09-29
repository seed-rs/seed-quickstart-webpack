
const binaryen = require("binaryen");
const fs = require("fs");
const findFiles = require('find')

// Optimize for size.
// https://www.npmjs.com/package/binaryen#module-optimization
binaryen.setShrinkLevel(2)

/**
 * Load, optimize and save all .wasm files in folder `dist`.
 */
findFiles.eachfile(/\.wasm$/, './dist', file => {
    const wasmModule = binaryen.readBinary(fs.readFileSync(file))
    wasmModule.optimize()
    fs.writeFileSync(file, wasmModule.emitBinary())
    console.log(`File: '${file}' optimized.`)
})
