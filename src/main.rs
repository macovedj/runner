use std::{fs, path};
use wasmtime::{
  component::{Component, Linker, TypedFunc, Func, Val},
  Config, Engine, Store,
};
use anyhow::Result;

pub fn main() -> Result<()> {
  let mut config = Config::new();
  config.wasm_component_model(true);
  let engine = Engine::new(&config)?;
  // let path = path::Path::new("/Users/interpretations/projects/witty/my-component.wasm");
  let contents = fs::read("comp.wasm")
         .expect("Something went wrong reading the file");
  // println!("PRINTING OUT SOME THIGNS");
  // println!("THE ACTUAL CONTENTS {:?}", contents);
  // let compWat = format!(
  //   r#"
  //   (component
  //     (core module (;0;)
  //       (type (;0;) (func (param i32 i32) (result i32)))
  //       (func (;0;) (type 0) (param i32 i32) (result i32)
  //         local.get 0
  //         local.get 1
  //         i32.add
  //       )
  //       (export "addTwo" (func 0))
  //     )
  //     (core instance (;0;) $foo (instantiate 0))
  //     (alias $falias core export 0 "apply" (core func $modfunc))
  //     (type $cftype (func (param "a" u32) (param "b" u32) (result u32)))
  //     (func $compfunc (type $cftype) (canon lift (core func $falias) string-encoding=utf8 (memory $malias)))
  //     (export "apply" (func 0))
  //   )
  //   "#);
  let compWat = format!(
    r#"
    (component
      (core module $mod
        (type $mftype (func (result i32)))
        (func $modfunc (type $mftype) (result i32)
          i32.const 0
        )
        (memory $modmem 1)
        (export "apply" (func 0))
        (export "mem" (memory 0))
        (data $return_area (i32.const 0) "\08\00\00\00\1a\00\00\00")
        (data $template_data (i32.const 8) "Lorem ipsum dolor sit amet")
      )
      (core instance $mod (instantiate 0))
      (alias core export 0 "apply" (core func $modfunc))
      (alias core export 0 "mem" (core memory $modmem))
      (type $cftype (func (result string)))
      (func $compfunc (type $cftype) (canon lift (core func $modfunc) string-encoding=utf8 (memory $modmem)))
      (export "apply" (func 0))
    )
    "#);
  let comp = Component::new(&engine, compWat)?;
  // println!("Output: {}", wasmprinter::print_bytes(&comp).unwrap());
  let linker = Linker::new(&engine);
  let mut store = Store::new(&engine, ());
  let instance = linker.instantiate(&mut store, &comp)?;
  // let func = instance.get_func(store, "addTwo");
  let func: TypedFunc<(), (String,)> = instance.get_typed_func(&mut store, "apply")?;
  let result = func.call(&mut store, ())?.0;
  println!("THESE ARE THE EXPORTS {:?}", result);
  // let apply: Func = 
  // let func: TypedFunc<(u8, u8), ()>  = instance.get_typed_func(&mut store, "addTwo")?;
  // // if let Some(f) = func {
  //     let left = Val::U8(2);
  //     let right = Val::U8(3);
  //     let results: &mut Vec<Val> = &mut Vec::new();
  //     func.call(&mut store, (2, 3));
  //     println!("THE RESULTS {:?}", results);
  // }
  // .map(|val| {
  // });
  // let result = apply.call(&mut store, (2, 3))?.0;
  
  Ok(())
}