// -----------------------------------------------------------------------------
//   - Basic view -
//   Load and display a template
// -----------------------------------------------------------------------------
use std::fs::read_to_string;

use anathema::runtime::Runtime;
use anathema::vm::Templates;
use anathema_runtime::RuntimeOptions;

fn main() {
    // Step one: Load and compile templates
    let template = read_to_string("examples/templates/basic.tiny").unwrap();
    let mut templates = Templates::new(template, ());
    let templates = templates.compile().unwrap();

    // Step two: Runtime
    let runtime = Runtime::new(&templates, RuntimeOptions::default()).unwrap();

    // Step three: start the runtime
    runtime.run().unwrap();
}
