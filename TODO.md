## LEXER
* traits for a Lexemes struct as nom input (https://github.com/Geal/nom/blob/master/doc/custom_input_types.md)
* lexeme tag parser
* trivially parse the tree with lexeme tags and combinators on top?!

## cleanup
* function for mode handling in interp/lex/parse

## features
* pinpoint error messages
* builtins (cd!)
* "shell stuff" (pipes, streams, job control, globs, conditionals, etc...)
* escape chars and other literals fun (make it legit)
* comments
* re-parse output
* literals for s[] l[] d[]
* functions
* loops
* scopes
* set notation (s[x | x in ys, x > 2])
* (full?) library of syscalls
* config (ps1, aliases, colors, etc.)

## crazy
* compilation
* (full!) library of syscalls
* shell mode POSIX compliant or close
