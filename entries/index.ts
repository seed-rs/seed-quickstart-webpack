import "../css/styles.css";

(async () => {
  (window as any).counter = await import("../components/counter/pkg/index");
  // Note: files in `crate/pkg/` will be created on the first build.
  await import("../crate/pkg/index");
})();
