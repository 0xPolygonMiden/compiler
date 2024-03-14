;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    (func $main (result i32)
        i32.const 2
        if (result i32)
            i32.const 3
        else
            i32.const 5
        end
    )
)

;; CHECK: (module #if_then_else
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #main) (result i32)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v1 i32) (const.i32 2))
;; CHECK-NEXT:             (let (v2 i1) (neq v1 0))
;; CHECK-NEXT:             (condbr v2 (block 2) (block 4)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1 (param v0 i32)
;; CHECK-NEXT:             (ret v0))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 2
;; CHECK-NEXT:             (let (v4 i32) (const.i32 3))
;; CHECK-NEXT:             (br (block 3 v4)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 3 (param v3 i32)
;; CHECK-NEXT:             (br (block 1 v3)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 4
;; CHECK-NEXT:             (let (v5 i32) (const.i32 5))
;; CHECK-NEXT:             (br (block 3 v5)))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
