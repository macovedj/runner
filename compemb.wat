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