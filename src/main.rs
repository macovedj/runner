use std::{fs, path};
use wasmtime::{
  component::{Component, Linker, TypedFunc, Func, Val},
  Config, Engine, Store,
};
use anyhow::Result;

const REALLOC_AND_FREE: &str = r#"
(global $last (mut i32) (i32.const 8))
(func $realloc (export "realloc")
    (param $old_ptr i32)
    (param $old_size i32)
    (param $align i32)
    (param $new_size i32)
    (result i32)
    (local $ret i32)
    ;; Test if the old pointer is non-null
    local.get $old_ptr
    if
        ;; If the old size is bigger than the new size then
        ;; this is a shrink and transparently allow it
        local.get $old_size
        local.get $new_size
        i32.gt_u
        if
            local.get $old_ptr
            return
        end
        ;; otherwise fall through to allocate a new chunk which will later
        ;; copy data over
    end
    ;; align up `$last`
    (global.set $last
        (i32.and
            (i32.add
                (global.get $last)
                (i32.add
                    (local.get $align)
                    (i32.const -1)))
            (i32.xor
                (i32.add
                    (local.get $align)
                    (i32.const -1))
                (i32.const -1))))
    ;; save the current value of `$last` as the return value
    global.get $last
    local.set $ret
    ;; bump our pointer
    (global.set $last
        (i32.add
            (global.get $last)
            (local.get $new_size)))
    ;; while `memory.size` is less than `$last`, grow memory
    ;; by one page
    (loop $loop
        (if
            (i32.lt_u
                (i32.mul (memory.size) (i32.const 65536))
                (global.get $last))
            (then
                i32.const 1
                memory.grow
                ;; test to make sure growth succeeded
                i32.const -1
                i32.eq
                if unreachable end
                br $loop)))
    ;; ensure anything necessary is set to valid data by spraying a bit
    ;; pattern that is invalid
    local.get $ret
    i32.const 0xde
    local.get $new_size
    memory.fill
    ;; If the old pointer is present then that means this was a reallocation
    ;; of an existing chunk which means the existing data must be copied.
    local.get $old_ptr
    if
        local.get $ret          ;; destination
        local.get $old_ptr      ;; source
        local.get $old_size     ;; size
        memory.copy
    end
    local.get $ret
)
"#;
pub fn main() -> Result<()> {
  let mut config = Config::new();
  config.wasm_component_model(true);
  let engine = Engine::new(&config)?;
  // let path = path::Path::new("/Users/interpretations/projects/witty/my-component.wasm");
  let contents = fs::read("comp.wasm")
         .expect("Something went wrong reading the file");
  let compWat = format!(
    r#"
    (component
      (core module (;0;)
        (type (;0;) (func (param i32 i32) (result i32)))
        (func (;0;) (type 0) (param i32 i32) (result i32)
          local.get 0
          local.get 1
          i32.add
        )
        (export "comp-add" (func 0))
      )
      (core instance (;0;) (instantiate 0))
      (type (;0;) (func (param "left" u32) (param "right" u32) (result u32)))
      (alias core export 0 "comp-add" (core func (;0;)))
      (func (;0;) (type 0) (canon lift (core func 0)))
      (export (;1;) "comp-add" (func 0))
    )
    "#);
    // (component
    //   (core module (;0;)
    //     (type (;0;) (func (param i32 i32) (result i32)))
    //     (func (;0;) $addId (type 0) (param i32 i32) (result i32)
    //       local.get 0
    //       local.get 1
    //       i32.add
    //     )
    //     (export "addTwo" (func 0))
    //   )
    //   (core instance (;0;) (instantiate 0))
    //   (alias core export 0 "addTwo" (core func $addId))
    //   (type $addType (func (param "left" u32) (param "right" u32) (result u32)))
    //   (func (type $addType) (canon lift (core func $addId)))
    //   (export "comp-add" (func 0))
    // )
    // (component
    //   (core module $mod
    //     (type $mftype (func (result i32)))
    //     (func $modfunc (type $mftype) (result i32)
    //       i32.const 0
    //     )
    //     (memory $modmem 1)
    //     (export "apply" (func 0))
    //     (export "mem" (memory 0))
    //     (data $return_area (i32.const 0) "\08\00\00\00\1a\00\00\00")
    //     (data $template_data (i32.const 8) "Lorem ipsum dolor sit amet")
    //   )
    //   (core instance $mod (instantiate 0))
    //   (alias core export 0 "apply" (core func $modfunc))
    //   (alias core export 0 "mem" (core memory $modmem))
    //   (type $cftype (func (result string)))
    //   (func $compfunc (type $cftype) (canon lift (core func $modfunc) string-encoding=utf8 (memory $modmem)))
    //   (export "apply" (func 0))
    // )
  let comp = Component::new(&engine, compWat)?;
  // println!("Output: {}", wasmprinter::print_bytes(&comp).unwrap());
  let linker = Linker::new(&engine);
  let mut store = Store::new(&engine, ());
  let instance = linker.instantiate(&mut store, &comp)?;
  // let func = instance.get_func(store, "addTwo");
  let func: TypedFunc<(u32, u32), (u32,)> = instance.get_typed_func(&mut store, "comp-add")?;
  let result = func.call(&mut store, ((2, 3)))?.0;
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