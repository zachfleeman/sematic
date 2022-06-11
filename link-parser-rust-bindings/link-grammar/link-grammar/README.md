
Directories
===========
Listed in rough processing order.

 * dict-common  - generic dictionary-handling code.
 * dict-file    - read dictionaries from files.
 * dict-sql     - read dictionaries from an SQL DB
                  (unfinished, under development!).

 * tokenize     - Convert sentence strings into word sequences.
                  Actually, into a word-graph of possible words.

 * prepare      - After tokenization, prepare sentence for parsing.

 * parse        - Classic Sleator/Temperley/Lafferty parser.

 * minisat      - Copy of the mini-SAT Boolean-SAT solver.
 * sat-solver   - Alternate parser, using boolean-SAT.

 * linkage      - Construction and analysis of linkages from parsing.

 * post-process - Constraint rules applied after parsing.
                  Also, rules for constituent-tree construction.

 * print        - functions that convert parses into
                  human-readable strings and printed output.


Version 5.3.14 - Improved error notification facility
=====================================================

This code is still "experimental", so it's API may be changed.

It is intended to be mostly compatible. It supports multi-threading -
all of its operations are local per-thread.
A visible change is that the first parameter of `prt_error()` should now end
with a newline in order to actually issue a message. However, its previous
auto-issuing of a newline was not documented.

Features:
---------
- Ability to intercept error messages (when required). This allows printing
them not only to stdout/stderr, but to any other stream (like logging)
or place (like a GUI window). This also allows to reformat the message.

- Possibility to print a message in parts and still have it printed as one
 complete message. The API for that is natural - messages are gathered
until a newline (if a message ends with `\n\\` this is an embedded
newline). The severity level of the last part, if exists, is used for the
whole message.

- New _severity levels_:
  * **Trace** (for `lgdebug()`).
  * **Debug** (for other debug messages).
  * **None** (for plain messages that need to use the error facility).

C API:
------
1)  `lg_error_handler lg_error_set_handler(lg_error_handler, void *data);`

Set an error handler function. Return the previous one.
On first call it returns the default handler function, that is
pre-installed on program start.
If the error handler is set to `NULL`, the messages are just queued,
and can be retrieved by `lg_error_printall()` (see (4) below).

For the default error handler, if data is not NULL, it is an
`(int *)` severity_level. Messages with <= this level are printed to stdout.
The default is to print Debug and lower to stdout.
For custom error handler it can be of course a more complex user-data.

2)  `const void *lg_error_set_handler_data(void * data);`

Return the current error handler user-data.
(This function is useful mainly for implementing correct language
bindings, which may need to free previously-allocated user-data).

3)  `char *lg_error_formatmsg(lg_errinfo *lge);`

Format the argument message.
It adds `link-grammar` and severity.
The `lg_errinfo` struct is part of the API.

4)  `int lg_error_printall(lg_error_handler, void *data);`

Print all the queued error messages and clear the queue.
Return the number of messages.

5)  `int lg_error_clearall(void);`
Clear the queue of error messages.
Return the number of messages.

6)  `int prt_error(const char *fmt, ...);`
Previously it was a void function, but now it returns an `int` (always 0) so
it can be used in complex macros (which make it necessary to use the comma
operator).

`prt_error()` still gets the severity label as a message prefix.
The list of error severities is defined as part of the API, and the
prefix that is used here is without the `lg_` part of the corresponding
enums.  The default severity is **None"**, i.e. a plain message.
(However, the enum severity code can be specified with the internal API
`err_msg()`. When both are specified, the label takes precedence. All of
these ways have their use in the code.)

Issuing a message in parts is supported. The message is collected until
its end and issued as one complete message. Issuing an embedded newline is
supported. In addition to a newline in a middle of string, which doesn't
terminate the message, and ending `\n\\` is a embedded newline.
This allows, for example, constructing a single message using a loop or
conditionals.

7)  `bool lg_error_flush(void);`
If a partial error message is buffered, flush it by adding a "\n" to it.
If no error message is buffered - do nothing.
Return **true** iff there was a message to flush.

See [link-includes.h](link-includes.h) for the definition of
severity levels and the `lg_errinfo` structure.

Notes:
------
1.  `lgdebug()` (used internally to issue debug or informational messages at
a given verbosity level) now usually uses by default the new severity level
`lg_Trace` but can instead use other levels (currently it sometimes uses
`lg_Debug` or `lg_Info)`.

2.  Some messages from the library may still use `printf()`, and the
intention is to convert them too to use the new error facility.

Language bindings:
------------------
A complete Python binding is provided under `class LG_Error`:
```
LG_Error.set_handler()
LG_Error.printall()
LG_Error.clearall()
LG_Error.message()     # prt_error()
errinfo.formatmsg()    # errinfo is the first argument of the error handler
errinfo.severity, errinfo.severity_label, errinfo.text # lg_errinfo fields
```

`class LG_Error` is also used as a general exception.
See [tests.py](../bindings/python-examples/tests.py) for usage of all of these
bindings.

SAT Solver
==========
The default Link Grammar parser constructs planar typed graphs from
a collection (dictionary) of graph-sheaf components (the "jigsaw puzzle
pieces") using an algorithm appropriate to this specific task. This
can be seen as a kind of constraint satisfaction problem: select
jigsaw puzzle pieces from the dictionary, such that they can be
assembled into a consistent whole.

The SAT solver was an experiment to see if performance could be
improved by applying the principles and algorithms of Boolean
Satisfiability Theory to this constraint satisfaction problem.

The result of the experiment was ultimately negative: early results
showed that the default parser is a bit faster for short sentences
and that the SAT solver can be faster for long sentences. Subsequent
enhancements to the original algorithm shows that it wins in all
situations.  Basically, the SAT solver cannot make effective use
of the planarity constraints that the default parser leverages to
discard impossible parses.

As of version 5.9.0, the SAT solver parser is not just a little bit
slower, it is a **LOT** slower than the default parser. It is:
* 5x slower on `corpus-basic.batch`.
* 4x slower on `corpus-fixes.batch`.
* More than 100x slower on `corpus-fix-long.batch`.
* 21x slower on Jane Austen's *Pride and Prejudice*.

The SAT solver speed can be significantly increased by these changes:
- Improve the XOR encoding. It has been tested, yielding a ~2x speedup for
  long sentences.
- Use connector sharing. This change alone has sped up the classic parser
  on batch parsing of `data/en/corpus-fix-long.batch` by 15x, but has less
  speedup potential for the SAT solver because it doesn't use connectors
  when it solves the SAT equations.
- Use the `power_prune()` function of the classic parser instead of its own
  `power_prune()`. This needs a tricky addition to delete connectors in the
  word expressions according the discarded disjuncts. It has been
  tested.
- Use "tracons". See `disjunct-utils.c` for what they are.
- Use memory pools.
- Improve and add hashing.
- Improve the postprocessing efficiency. For short sentences (also in
  the classic parser) this has a potential for maybe 10% speedup.
  However, for getting several parsings for long sentences a huge speedup
  is expected.

The code can still be built by saying
```
configure --enable-sat-solver
```
but is now disabled by default. The SAT solver code may be removed in
future versions.

If the `minisat2` library package is installed in the system along with
its header files (e.g. RPM package `minisat2-devel`, deb package
`minisat2`), then it is used. Else, the bundled `minisat2` library code
is used.

One can force the bundled version to always be used by saying:
```
./configure --enable-sat-solver=bundled
```

Other problems with the SAT solver include:
- Cannot parse with null links (can be fixed but it is not trivial).
- No panic timeout (trivial to fix).
