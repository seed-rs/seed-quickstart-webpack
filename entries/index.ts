import "../css/styles.css";

(async () => {
  // Note: files in `crate/pkg/` will be created on the first build.
  await import("../crate/pkg/index");
})();
