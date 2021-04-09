module.exports = ({ file, options, env }) => {

  return {
    plugins: [
      require("postcss-import"),
      require("tailwindcss")('tailwind.config.js'),
      require("postcss-typed-css-classes")({
        generator: "rust",
        purge: env === "production",
        output_filepath: "src/generated/css_classes.rs",
        content: [
          { path: ['src/**/*.rs'] },
          {
            path: ['static/index.hbs', 'static/templates/**/*.hbs'],
            regex: /class\s*=\s*["'|][^"'|]+["'|]/g,
            mapper: className => {
              return (className.match(/class\s*=\s*["'|]([^"'|]+)["'|]/)[1]).match(/\S+/g)
            },
            escape: true
          }
        ],
      }),
      require("autoprefixer")
    ]
  };
};
