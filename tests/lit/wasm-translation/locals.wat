;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    (func $main (local i32)
        i32.const 1
        local.set 0
        local.get 0
        drop
    )
)

;; CHECK: (module #locals
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #main)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v0 i32) (const.i32 0))
;; CHECK-NEXT:             (let (v1 i32) (const.i32 1))
;; CHECK-NEXT:             (br (block 1)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1
;; CHECK-NEXT:             (ret))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
