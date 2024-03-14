;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    (func $add (param i32 i32) (result i32)
        local.get 0
        local.get 1
        i32.add
    )
    (func $main (result i32)
        i32.const 3
        i32.const 5
        call $add
    )
)

;; CHECK: (module #func_call
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #add) (param i32) (param i32) (result i32)
;; CHECK-NEXT:         (block 0 (param v0 i32) (param v1 i32)
;; CHECK-NEXT:             (let (v3 i32) (add.wrapping v0 v1))
;; CHECK-NEXT:             (br (block 1 v3)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1 (param v2 i32)
;; CHECK-NEXT:             (ret v2))
;; CHECK-NEXT:     )
;; CHECK-EMPTY:
;; CHECK-NEXT:     (func (export #main) (result i32)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v1 i32) (const.i32 3))
;; CHECK-NEXT:             (let (v2 i32) (const.i32 5))
;; CHECK-NEXT:             (let (v3 i32) (call #add v1 v2))
;; CHECK-NEXT:             (br (block 1 v3)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1 (param v0 i32)
;; CHECK-NEXT:             (ret v0))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
