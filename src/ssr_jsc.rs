use rusty_jsc::JSContext;

use crate::polyfill;

#[derive(Clone, Debug, PartialEq)]
pub struct SsrJsc<'a> {
    source: String,
    entry_point: &'a str,
}

impl<'a> SsrJsc<'a> {
    pub fn new(source: String, entry_point: &'a str) -> Self {
        SsrJsc {
            source,
            entry_point,
        }
    }

    pub fn one_shot_render(source: String, entry_point: &str, params: Option<&str>) -> String {
        Self::render(source, entry_point, params)
    }

    pub fn render_to_string(&self, params: Option<&str>) -> String {
        Self::render(self.source.clone(), self.entry_point, params)
    }

    // TODO: params for initial props
    fn render(source: String, entry_point: &str, _params: Option<&str>) -> String {
        let mut context = JSContext::default();

        let prefix = polyfill::POLYFILL.as_str();

        // TODO: destruct the object
        let eval = &format!("{};{};{}", prefix, source, entry_point);

        let value = context.evaluate_script(eval.as_str(), 1);

        let mut rendered = String::new();

        if let Some(value) = value {
            rendered = value.to_string(&context);
        }

        rendered
    }
}
