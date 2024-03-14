;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    (func $main (result i32) (local i32)
        block
            i32.const 3
            local.set 0
            br 0
        end
        local.get 0
    )
)

;; CHECK: (module #br
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #main)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v1 i32) (const.i32 0))
;; CHECK-NEXT:             (let (v2 i32) (const.i32 3))
;; CHECK-NEXT:             (br (block 2)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1 (param v0 i32)
;; CHECK-NEXT:             (ret v0))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 2
;; CHECK-NEXT:             (br (block 1 v2))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
