(module $mod
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