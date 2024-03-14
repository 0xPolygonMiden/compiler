;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    (global $MyGlobalVal (mut i32) i32.const 42)
    (func $main
        global.get $MyGlobalVal
        i32.const 9
        i32.add
        global.set $MyGlobalVal
    )
)

;; CHECK: (module #global_var
;; CHECK-NEXT:     ;; Constants
;; CHECK-NEXT:     (const (id 0) 0x0000002a)
;; CHECK-EMPTY:
;; CHECK-NEXT:     ;; Global Variables
;; CHECK-NEXT:     (global (export #MyGlobalVal) (id 0) (type i32) (const 0))
;; CHECK-EMPTY:
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #main)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v0 i32) (global.load i32 (global.symbol #MyGlobalVal)))
;; CHECK-NEXT:             (let (v1 i32) (const.i32 9))
;; CHECK-NEXT:             (let (v2 i32) (add.wrapping v0 v1))
;; CHECK-NEXT:             (let (v3 (ptr i32)) (global.symbol #MyGlobalVal))
;; CHECK-NEXT:             (store v3 v2)
;; CHECK-NEXT:             (br (block 1)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1
;; CHECK-NEXT:             (ret))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
