
const binaryen = require("binaryen");
const fs = require("fs");
const findFiles = require('find')

// optimize for size
// https://www.npmjs.com/package/binaryen#module-optimization
binaryen.setShrinkLevel(2)

findFiles.eachfile(/\.wasm$/, './dist', file => {
    const wasmModule = binaryen.readBinary(fs.readFileSync(file))
    wasmModule.optimize()
    fs.writeFileSync(file, wasmModule.emitBinary())
})
