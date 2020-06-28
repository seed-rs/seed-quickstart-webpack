import "./styles/app.scss";

(async () => {
  // Note: files in `./pkg/` will be created on the first build.
  await import("./pkg/index");
})();
