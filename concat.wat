(module (
  (func (export "realloc") (param $originalPtr i32)
      (param $originalSize i32)
      (param $alignment i32)
      (param $newSize i32)
      (result i32) (
        local.get $originalSize
      )
  )

  (func (export "concat") (param i32 i32 i32 i32) (result i32) (
    local.get 0
    local.get 1
    i32.add
    local.get 2
    local.get 3
    i32.add
    
  ))
))