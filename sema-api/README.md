# sema-api
Main API for the Sema project


## Local Development

This project uses [auto reloading](https://actix.rs/docs/autoreload/) in dev.

You must have `cargo-watch` installed on your system

    cargo install cargo-watch


To recompile and rerun on source or config changes

    cargo watch -x run --clear --no-gitignore

### Dealing with a "error: EADDRINUSE: Address already in use"

https://stackoverflow.com/questions/3855127/find-and-kill-process-locking-port-3000-on-mac

NOTE: shut down the web client and the caddy server BEFORE running the kill command below.

```
$ sudo lsof -i :8088
// or
$ sudo lsof -i tcp:8088
// the kill
$ kill -9 <pid>
```

### Link grammar pos "subscripts"

```
.a // Adjective

.a-c // Adjective, comparative/relative

.a-s // Adjective, superlative

.b // Given names that can be masculine or feminine

.c // Currency names

.d // (Not used/not defined)

.e // Adverbs

.eq .eqn // Binary operators e.g. 2 + 2

.f // Given names that are always feminine

.g //Gerund

.h // Hesitation markers, fillers, planners

.i // Misc usage, mostly pertaining to units, lengths, times.

.id // Identifiers: e.g. "vitamin A"

.ij // Interjections, fillers

.j // Conjunctions.

.j-a // Conjunctions -- adjectives: "the black and white cat"

.j-c // Conjunctions -- comparatives: "he is bigger, and badder, than the pope."

.j-g // Conjunctions - proper names: e.g. "The Great Southern and Western Railroad"

.j-m // Conjunctions -- post-nominal modifiers

.j-n // Conjunctions -- nouns: "Jack and Jill"

.j-o // Conjunctions -- ditransitive e.g. "I gave Bob a doll and Mary a gun"

.j-opnr // Clause openers -- e.g. "but you are wrong!"

.j-q // Conjunctions -- Conjoined question words.

.j-r // Conjunctions -- adverbs/prepositional phrases e.g. "the man for whom and with whom ..."

.j-ru // Conjunctions -- interval e.g. "two to threefold more abundant"

.j-sum // Conjunctions -- numerical sums: e.g. "It's a hundred and two in the shade."

.j-v // Conjunctions -- verbs: "sang and danced"

.k // (Not used/not defined)

.l // Location (cities, states, towns, etc.)

.m // Given names that are always masculine

.n // Noun

.n-u // Noun, uncountable (mass noun)

.o // Organizations (corporations)

.ord // Ordinal numbers e.g. first second third

.p // Plural count nouns

.q // verb, Question-related or paraphrasing

.q-d // verb, past tense

.r // Prepositions and related

.s // Singular, mass or count nouns

.t // Titles, roles. e.g. President, Captain

.ti // Time, date e.g. AM, PM, December 2nd

.tz // Time-zones e.g. CDT, UTC

.u // Units of measurement

.v // Verb

.v-d // Verb, past tense

.w // Verb

.w-d // Verb, past tense

.x // Prefix abbreviations, e.g. Mr., Dr., Mrs.

.y // Postfix abbreviations, e.g. Ave., St., Co.

.z // (Not used/not defined)

```









































