// use std::fs::read_to_string;

// // use rusty_jsc::JSContext;
// use ssr_react::polyfill::POLYFILL;

// fn main() {
//     let mut context = JSContext::default();

//     let source = r##"var SSR; SSR = {x: () => "<html></html>"};"##.to_string();
//     let eval = format!("{};{}", source, "SSR.x();");
//     let value = context.evaluate_script(eval.as_str(), 1);
//     if let Some(value) = value {
//         println!("{}", value.to_string(&context));
//     }
// }

fn main() {
    // let mut context = JSContext::default();

    // let source = read_to_string("./examples/simple-ssr/source/dist/ssr/index.js").unwrap();
    // let eval = &format!(
    //     "{};{};{};",
    //     POLYFILL.as_str(),
    //     source,
    //     // "Object.values(SSR).map(e => e());"
    //     "SSR"
    // );
    // let value = context.evaluate_script(eval.as_str(), 1);
    // if let Some(value) = value {
    //     println!("{}", value.to_string(&context));
    // }
}
