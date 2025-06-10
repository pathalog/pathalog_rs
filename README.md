# pathalog_rs

## Concepts
* Universe
  * A Universe is an ordered list of all the tokens/list in the universe. The default Universe is ```()```
  * A Universe is the top level perspective that any problem/program is being looked at
  * Everything in a Universe is a list. A bare token not enclosed in parentheses is considered a single item list ```A=>(A)```
  * Universes can nest, and in fact, every list can be considered its own local universe
  * The first element of every list is a token that is accessible to the enclosing universe, since it's the head of it's own list
  * ```((A) (B) (C))``` defines a universe consisting of three tokens (A, B, and C), in that order
  * ```(A B C)``` this is identical to the list above due to single element parenethesis elision
* Sequences 
  * A sequence is expressed by a simple S-expression or list, such as ```(1 2 3)```
  * All sequences are iterators
  * All sequences can be considered the same as lisp CONS cells, with a head (current element) and a tail (the rest)
  * All sequences/lists are strongly ordered and cannot be randomly accessed. Only iterated throughk, in order.  
  * Consuming the head of a sequence returns the tail, with the head of the tail as your new hea
* Labels/Token
  * A label is the same thing as a token. It is a sequence of printable characters.
  * The head of 
* Iteration
* Evaluation
  * There is no evaluation, only Observation.
  * Anything that outputs a result is an observation
  * Hency all operations are lazy
* Defn
  * There are no functions, so no special syntax to define them
* Symbols are implicitly defined by putting them at the beginning of a list. They are then defined as an alias for the tail of that list
* Observing a symbol is done (for now, by appending an exclamation to that symbol. That both simplifies the symbol as well as sends it to stdout
