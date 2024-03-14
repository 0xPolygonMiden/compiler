;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    ;; Sum the decreasing numbers from 2 to 0, i.e. 2 + 1 + 0, then exit the loop
    (func $main (result i32) (local i32 i32)
        i32.const 2
        local.set 0
        loop
            local.get 0
            local.get 1
            i32.add
            local.set 1
            local.get 0
            i32.const 1
            i32.sub
            local.tee 0
            br_if 0
        end
        local.get 1
    )
)

;; CHECK: (module #loop_br_if
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #main) (result i32)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v1 i32) (const.i32 0))
;; CHECK-NEXT:             (let (v2 i32) (const.i32 2))
;; CHECK-NEXT:             (br (block 2 v2 v1)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1 (param v0 i32)
;; CHECK-NEXT:             (ret v0))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 2 (param v3 i32) (param v4 i32)
;; CHECK-NEXT:             (let (v5 i32) (add.wrapping v3 v4))
;; CHECK-NEXT:             (let (v6 i32) (const.i32 1))
;; CHECK-NEXT:             (let (v7 i32) (sub.wrapping v3 v6))
;; CHECK-NEXT:             (let (v8 i1) (neq v7 0))
;; CHECK-NEXT:             (condbr v8 (block 2 v7 v5) (block 4)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 3
;; CHECK-NEXT:             (br (block 1 v5)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 4
;; CHECK-NEXT:             (br (block 3)))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
