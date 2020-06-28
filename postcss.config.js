module.exports = ({ file, options, env }) => {
  return {
    plugins: [
      require("postcss-typed-css-classes")({
        generator: "rust",
        purge: options.mode === "production",
      }),
      options.mode === "production" && require('cssnano')({
        "preset": [
          "default",
          { "discardComments": { "removeAll": true } }
        ]
      }),
      require("autoprefixer")
    ]
  };
};
