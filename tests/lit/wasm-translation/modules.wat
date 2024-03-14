;; RUN: bin/midenc compile --stdout --emit=hir %s | filecheck %s
(module
    (func $main
        i32.const 0
        drop
    )
)

;; CHECK: (module #modules
;; CHECK-NEXT:     ;; Functions
;; CHECK-NEXT:     (func (export #main)
;; CHECK-NEXT:         (block 0
;; CHECK-NEXT:             (let (v0 i32) (const.i32 0))
;; CHECK-NEXT:             (br (block 1)))
;; CHECK-EMPTY:
;; CHECK-NEXT:         (block 1
;; CHECK-NEXT:             (ret))
;; CHECK-NEXT:     )
;; CHECK-NEXT: )
