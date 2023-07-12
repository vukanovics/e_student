use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
    ScopedJson,
};

use handlebars::to_json;

pub struct ConcatHelper {}

impl HelperDef for ConcatHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let mut result = String::new();

        for param in h.params() {
            // trying to get the param as a proper string, but failing that,
            // give me literally anything
            // lets us concat numbers with strings
            let value = param.value();
            let value = value
                .as_str()
                .map(|s| s.to_owned())
                .unwrap_or_else(|| value.to_string());

            result += &value;
        }

        out.write(&result)?;
        Ok(())
    }
}

impl ConcatHelper {
    pub fn name() -> &'static str {
        "concat"
    }

    pub fn helper() -> Box<dyn HelperDef + Send + Sync + 'static> {
        Box::new(ConcatHelper {})
    }
}

pub struct EqHelper {}

impl HelperDef for EqHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let first = h
            .param(0)
            .ok_or(RenderError::new("missing first parameter"))?
            .value();
        let second = h
            .param(1)
            .ok_or(RenderError::new("missing second parameter"))?
            .value();

        Ok(ScopedJson::Derived(to_json(first == second)))
    }
}

impl EqHelper {
    pub fn name() -> &'static str {
        "eq"
    }

    pub fn helper() -> Box<dyn HelperDef + Send + Sync + 'static> {
        Box::new(EqHelper {})
    }
}
