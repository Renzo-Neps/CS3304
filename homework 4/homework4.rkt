#lang racket

{provide asum find-indices remove square}


(define (square lst)
	(map (lambda (x) (* x x)) lst)
)

(define (remove n lst)
	(cond
		((null? lst) '())
		((equal? n (car lst)) (remove n (cdr lst)))
		(else (append (list (car lst)) (remove n (cdr lst))))
    )
)

(define (asum lst)
	(foldr (lambda (ele op acc) (op acc ele)) 0 lst
        (build-list (length lst)
                     (lambda (i) (if (even? i) + -))
		)
	)
)

(define (find-indices n lst)
	(let loop ((lst lst) (idx 0))
		(cond
			((empty? lst) '())
			((equal? n (first lst)) (append (list idx) (loop (rest lst) (add1 idx))))
			(else (loop (rest lst) (add1 idx)))
		)
	)
)