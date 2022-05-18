unit sub MAIN ($expression where $expression
                  ~~ /^<[0..9 \( \) \+ \- \* \s]>+$/);  # [1]
say $expression.EVAL;