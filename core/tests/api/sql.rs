// use deno_core::op;
// use deno_core::Extension;
// use deno_core::JsRuntime;
// use deno_core::RuntimeOptions;

// #[test]
// fn main3() {
//     let mut isolate = &mut JsRuntime::new(RuntimeOptions::default());

//     {
//         let scope = &mut isolate.handle_scope();
//         let context = scope.get_current_context();
//         let global = context.global(scope);
//         let scope = &mut v8::ContextScope::new(scope, context);

//         let test_key = v8::String::new(scope, "test").unwrap();
//         let test_val = v8::Object::new(scope);

//         let my_func_key = v8::String::new(scope, "my_func").unwrap();
//         let my_func_templ = v8::FunctionTemplate::new(scope, my_func);
//         let my_func_val = my_func_templ.get_function(scope).unwrap();

//         global.set(scope, my_func_key.into(), my_func_val.into());

//         global.set(scope, test_key.into(), test_val.into());

//         // scope.escape(context);
//     }



//     let v = isolate.execute_script_static("<usage>", "my_func();");

//     match v {
//         Ok(val) => println!("sucessfuly completed {:?}", val),
//         Err(val) => println!("Error {:?}", val),
//     }
// }

// fn my_func(
//     scope: &mut v8::HandleScope,
//     args: v8::FunctionCallbackArguments,
//     mut _rv: v8::ReturnValue,
// ) {
//     println!("In my func");
//     let arg1: String = match serde_v8::from_v8(scope, args.get(0)) {
//         Ok(zbuf) => zbuf,
//         Err(_) => {
//             //throw_type_error(scope, "Invalid argument");
//             return;
//         }
//     };
//     let arg2: String = match serde_v8::from_v8(scope, args.get(1)) {
//         Ok(zbuf) => zbuf,
//         Err(_) => {
//             //throw_type_error(scope, "Invalid argument");
//             return;
//         }
//     };

//     // do something...
// }