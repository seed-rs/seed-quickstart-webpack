const postcssRustHelpers = require("../scripts/postcss_rust_helpers");

module.exports = ({ file, options, env }) => {
  // we want to filter out unused css classes in production mode
  // NOTE: options.mode is set in webpack configs, in the postcss loader ctx
  const usedCssClasses =
    options.mode === "production"
      ? postcssRustHelpers.getUsedCssClasses()
      : null;

  return {
    plugins: [
      require("postcss-import"),
      require("tailwindcss")('configs/tailwind.config.js'),
      require("postcss-typed-css-classes")({
        output_filepath: "crate/src/generated/css_classes.rs",
        generator: "rust",
        filter: class_ => {
          if (options.mode === "production") {
            return usedCssClasses.has(
              postcssRustHelpers.escapeClassName(class_)
            );
          } else {
            return true;
          }
        }
      }),
      require("autoprefixer")
    ]
  };
};
