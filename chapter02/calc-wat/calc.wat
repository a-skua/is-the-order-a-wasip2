(module
  (func $add (param $x i32) (param $y i32) (result i32)
    (i32.add (local.get $x) (local.get $y))
  )
  (export "component:calc/calc@0.1.0#add" (func $add))
)
