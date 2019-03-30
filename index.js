import("./css/styles.css");
import("./crate/pkg").then(module => {
  module.run();
});
