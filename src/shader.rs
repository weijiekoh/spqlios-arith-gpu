use minijinja::{Environment, context};
use std::path::PathBuf;

fn read_from_file(path: &str, file: &str) -> String {
    let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(path)
        .join(file);
    std::fs::read_to_string(&input_path).unwrap()
}

pub fn render_simple(template_file: &str) -> String {
    let tests_path: &str = "src/wgsl/tests";
    let source = read_from_file(tests_path, template_file);

    let mut env = Environment::new();
    env.add_template(template_file, &source).unwrap();

    let context = context! {};
    let template = env.get_template(template_file).unwrap();
    template.render(context).unwrap()
}
