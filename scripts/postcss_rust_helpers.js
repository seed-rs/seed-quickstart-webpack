const path = require('path')
const findFiles = require('find');
const fs = require('fs');

// Helpers are used by configs/postcss.config.js

module.exports = {
    /**
     * Search in Rust files for C.class_name
     *
     * @returns usedCssClasses Set of class names
     */
    getUsedCssClasses: function() {
        const usedCssClasses = new Set;
        // search in Rust files
        const files = findFiles.fileSync(/\.rs$/, './crate/src');
        files.forEach(filePath => {
            const fileContent = fs.readFileSync(filePath, 'utf8')
            // example of a used class in Rust code is `C.mb_16`
            usedCssClassesInFile = fileContent.match(/C\.[a-z0-9_]+/g)
            if(usedCssClassesInFile) {
                usedCssClassesInFile
                    // remove prefix `C.`
                    .map(class_ => class_.substring(2))
                    .forEach(class_ => usedCssClasses.add(class_))
            }
        })
        return usedCssClasses
    },
    /**
     * Transform css class name into indentifier that can be used as Rust struct field name
     * (see https://github.com/MartinKavik/postcss-typed-css-classes/blob/master/generators/rust_generator.js)
     *
     * @param {string} name
     * @returns string Escaped class name
     */
    escapeClassName: function (name) {
        name = name.replace(/-/g, '_')
        name = name.replace(/:/g, '__')
        name = name.replace(/\//g, 'of')
        if (getKeywords().indexOf(name) > -1) {
            name += '_'
        }
        return name
    }
}

// https://doc.rust-lang.org/book/appendix-01-keywords.html
function getKeywords () {
    return [
        'as',
        'break',
        'const',
        'continue',
        'crate',
        'dyn',
        'else',
        'enum',
        'extern',
        'false',
        'fn',
        'for',
        'if',
        'impl',
        'in',
        'let',
        'loop',
        'match',
        'mod',
        'move',
        'mut',
        'pub',
        'ref',
        'return',
        'Self',
        'self',
        'static',
        'struct',
        'super',
        'trait',
        'true',
        'type',
        'unsafe',
        'use',
        'where',
        'while',

        'abstract',
        'async',
        'become',
        'box',
        'do',
        'final',
        'macro',
        'override',
        'priv',
        'try',
        'typeof',
        'unsized',
        'virtual',
        'yield'
    ]
}
