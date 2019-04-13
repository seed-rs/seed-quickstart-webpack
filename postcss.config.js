const path = require('path')

module.exports = {
  plugins: [
    require("tailwindcss")("./tailwind.js"),
    require("postcss-typed-css-classes")({
      output_filepath: "./crate/src/generated/css_classes.rs",
      generator: "rust",
      filter: () => true
    }),
    require("autoprefixer")
  ]
};
