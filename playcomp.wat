(component
  (core module (;0;)
    (type (;0;) (func (param i32 i32) (result i32)))
    (func (;0;) (type 0) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.add
    )
    (export "addTwo" (func 0))
  )
  (core instance (;0;) (instantiate 0))
)