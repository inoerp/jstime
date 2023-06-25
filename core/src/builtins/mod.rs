use std::convert::TryFrom;


lazy_static! {
    pub(crate) static ref EXTERNAL_REFERENCES: v8::ExternalReferences =
        v8::ExternalReferences::new(&[
            v8::ExternalReference {
                function: v8::MapFnTo::map_fn_to(printer),
            },
            v8::ExternalReference {
                function: v8::MapFnTo::map_fn_to(queue_microtask),
            },
            v8::ExternalReference {
                function: v8::MapFnTo::map_fn_to(sql_select),
            },
        ]);
}

pub(crate) struct Builtins {}

impl Builtins {
    pub(crate) fn create(scope: &mut v8::HandleScope) {
        let bindings = v8::Object::new(scope);

        macro_rules! binding {
            ($name:expr, $fn:expr) => {
                let name = v8::String::new(scope, $name).unwrap();
                let value = v8::Function::new(scope, $fn).unwrap();
                bindings.set(scope, name.into(), value.into());
            };
        }

        binding!("printer", printer);
        binding!("queueMicrotask", queue_microtask);

        let context = v8::Context::new(scope);
        let global = context.global(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        //set key & values
        let deno_key = v8::String::new(scope, "Deno").unwrap();
        let deno_val = v8::Object::new(scope);
        global.set(scope, deno_key.into(), deno_val.into());
        
        let core_key = v8::String::new(scope, "core").unwrap();
        let core_val = v8::Object::new(scope);
        deno_val.set(scope, core_key.into(), core_val.into());
        set_func(scope, core_val, "sqlSelect", sql_select);

        // Handle the result if needed
    }
}

pub fn set_func(
    scope: &mut v8::HandleScope<'_>,
    obj: v8::Local<v8::Object>,
    name: &'static str,
    callback: impl v8::MapFnTo<v8::FunctionCallback>,
) {
    let key = v8::String::new(scope, name).unwrap();
    let tmpl = v8::FunctionTemplate::new(scope, callback);
    let val = tmpl.get_function(scope).unwrap();
    val.set_name(key);
    obj.set(scope, key.into(), val.into());
}

fn printer(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    // Implementation of the printer function goes here
}

fn queue_microtask(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    // Implementation of the queueMicrotask function goes here
}

fn sql_select(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    println!("in sql_select rust");
}
