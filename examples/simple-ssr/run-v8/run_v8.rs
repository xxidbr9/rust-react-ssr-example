//This example exist just for develop purposes
use ssr_react::polyfill::POLYFILL;
use std::fs::read_to_string;

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Create a new Isolate and make it the current one.
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

    // Create a stack-allocated handle scope.
    let handle_scope = &mut v8::HandleScope::new(isolate);

    // Create a new context.
    let context = v8::Context::new(handle_scope);

    // Enter the context for compiling and running the hello world script.
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    // Create a string containing the JavaScript source code.

    // let source = r##"var SSR; SSR = {x: () => "<html></html>"};"##.to_string();
    let source = read_to_string("./examples/simple-ssr/source/dist/ssr/index.js").unwrap();

    let code = v8::String::new(
        scope,
        &format!("{};{};{}", POLYFILL.as_str(), source, "SSR"),
    )
    .unwrap();

    // Compile the source code.
    let script = v8::Script::compile(scope, code, None).unwrap();
    // Run the script to get the result.
    let result = script.run(scope).unwrap();
    println!("{}", result.to_rust_string_lossy(scope));

    // let object = result.to_object(scope).expect("Error there is object");
    
}
