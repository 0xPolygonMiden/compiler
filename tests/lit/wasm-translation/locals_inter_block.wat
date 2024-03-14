;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    (func $main (result i32) (local i32)
        block
            i32.const 3
            local.set 0
        end
        block
            local.get 0
            i32.const 5
            i32.add
            local.set 0
        end
        i32.const 7
        local.get 0
        i32.add
    )
)

;; CHECK: (module #locals_inter_block
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #main) (result i32)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v1 i32) (const.i32 0))
;; CHECK-NEXT:             (let (v2 i32) (const.i32 3))
;; CHECK-NEXT:             (br (block 2)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1 (param v0 i32)
;; CHECK-NEXT:             (ret v0))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 2
;; CHECK-NEXT:             (let (v3 i32) (const.i32 5))
;; CHECK-NEXT:             (let (v4 i32) (add.wrapping v2 v3))
;; CHECK-NEXT:             (br (block 3)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 3
;; CHECK-NEXT:             (let (v5 i32) (const.i32 7))
;; CHECK-NEXT:             (let (v6 i32) (add.wrapping v5 v4))
;; CHECK-NEXT:             (br (block 1 v6)))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
