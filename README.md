# lexper
lexper is a small & lightweight calculator written in Rust with custom lexer and parser.  
It also supports constants and function calls(currently sin() and PI as constant).

You can trust me, it rans at +-466us with the following expression: "sin(1.2)^2 + 4.0 * (sin(1000) + sin(2.0)) * 10.5".  
Not so fast as I, probably, would like, but im satisfied with the results for now.
