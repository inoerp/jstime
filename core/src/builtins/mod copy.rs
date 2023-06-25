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
        println!("In_macro 1");
        let bindings = v8::Object::new(scope);

        macro_rules! binding {
            ($name:expr, $fn:ident) => {
                let name = v8::String::new(scope, $name).unwrap();
                let value = v8::Function::new(scope, $fn).unwrap();
                bindings.set(scope, name.into(), value.into());
            };
        }

        binding!("printer", printer);
        binding!("queueMicrotask", queue_microtask);
        binding!("sqlSelect", sql_select);
        println!("In_macro 2");
        macro_rules! builtin {
            ($name:expr) => {
                let source = include_str!($name);
                let val = match crate::script::run(scope, source, $name) {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                };
                let func = v8::Local::<v8::Function>::try_from(val).unwrap();
                let recv = v8::undefined(scope).into();
                let args = [bindings.into()];
               func.call(scope, recv, &args).unwrap();
            };
        }
        println!("In_macro 5");
        builtin!("./console.js");
        builtin!("./queue_microtask.js");
        builtin!("./sql_select.js");
    }
}

fn printer(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    let arg_len = args.length();
    assert!((0..=2).contains(&arg_len));

    let obj = args.get(0);
    let is_err_arg = args.get(1);

    let mut is_err = false;
    if arg_len == 2 {
        let int_val = is_err_arg
            .integer_value(scope)
            .expect("Unable to convert to integer");
        is_err = int_val != 0;
    };
    let tc_scope = &mut v8::TryCatch::new(scope);
    let str_ = match obj.to_string(tc_scope) {
        Some(s) => s,
        None => v8::String::new(tc_scope, "").unwrap(),
    };
    if is_err {
        eprintln!("{}", str_.to_rust_string_lossy(tc_scope));
    } else {
        println!("{}", str_.to_rust_string_lossy(tc_scope));
    }
}

fn queue_microtask(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let obj = args.get(0);
    let func = v8::Local::<v8::Function>::try_from(obj).unwrap();
    scope.enqueue_microtask(func);
}

fn sql_select(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    println!("In_macro 1.2");
    println!("in sql_select rust");
    println!("args in rust {:?}", args);
    let obj = args.get(0);
    

    // let tc_scope = &mut v8::TryCatch::new(scope);
    // let str_ = match obj.to_string(tc_scope) {
    //     Some(s) => s,
    //     None => v8::String::new(tc_scope, "").unwrap(),
    // };
    // let mut is_err = false;
    // if is_err {
    //     eprintln!("sql_select_rust {}", str_.to_rust_string_lossy(tc_scope));
    // } else {
    //     println!("sql_select_rust {}", str_.to_rust_string_lossy(tc_scope));
    // }
    args.new_target().integer_value(scope).insert(200);
    rv.set(v8::Number::new(scope, 55.0).into());

 
}
