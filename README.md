# Kidneyshit
An extension of Brainfuck (following the "organ-swear" naming rule).
Brainfuck was supposed to be as annoying as possible, but fuck that. Number literals FTW.
(almost) any BF code can be interpreted, however KS has many more additions.
Much more stack based than BF, as it seems that it's more efficient.

# Additions
Adds a stack.
Popping the stack will pad it with 0s.
The heap uses i32s, making some BF projects that overflow unable to interpret. However, this allows for higher limits, and allows you to have more options for output.
The heap also uses a vector, expanding on out-of-bounds.

# Instructions
Old BF:
+: Increment
-: Decrement
\>: Pointer left
<: Pointer right
[: Start loop (loops until heap value is 0)
]: End loop
.: Put char
,: Get char

New instructions:
=: Set number (e.g. =90 will set to 90).
$: Increment with number.
\#: Decrement with number.
{: Move pointer right with number.
}: Move pointer left with number.
(: Start stack loop (loops until stack is empty).
): End stack loop.
Stack loops are completely disconnected from heap loops. You can have 2 loops weaved like this: ( [ ) ] and it will still run.
!: Print number.
y: Sets the memory pointer to the current heap value.
t: Pushes a char onto the stack.
u: Puts a char into the heap.
w: Pops the stack, and pushes a string of that length onto the stack (first char is at the top).
r: Pops the stack, and reverses that many numbers.

;: Interprets the rest of this line as a comment.

^: Pops the stack.
v: Pushes a number onto the stack.
o: Pushes the current heap value onto the stack.
p: Pops the current stack value into the heap.
c: Copies the current stack value into the heap (doesn't pop).

Stack arithmetic will pop the left and right values.
s+: Adds the top two items in the stack (s prefix due to BF's +).
s-: Subtracts the top two items in the stack (s prefix due to BF's -).
*: Multiplies the top two items in the stack.
/: Divides the top two items in the stack.
\%: Modulos the top two items in the stack.

NOT PROPERLY IMPLEMENTED - DO NOT USE:
Functions: Triggered by a heap value, more versatile than macros, but slightly slower.
Macros: Replaced upon preinterpretation, triggered by a number literal rather than a heap value. Uses slightly more memory, but faster than functions.

md: Defines a macro spanning the rest of the line using a number as an index. If a macro with the same index is already defined, the macro is overwritten.
mt: Triggers a macro corresponding to a number.

fd: Defines a function spanning the rest of the line using the current heap value as the index. If a function with the same index is already defined, the function is overwritten.
ft: Triggers a function corresponding to the current heap value.
