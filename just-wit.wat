(component
  (type (;0;)
    (component
      (type (;0;)
        (component
          (type (;0;) (func (param "left" u32) (param "right" u32) (result u32)))
          (export (;0;) "comp-add" (func (type 0)))
        )
      )
      (export (;0;) "hello" "pkg:/playcomp/hello" (component (type 0)))
    )
  )
  (export (;1;) "playcomp" "pkg:/playcomp" (type 0))
)