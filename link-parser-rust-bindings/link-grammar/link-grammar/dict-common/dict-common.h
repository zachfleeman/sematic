/*************************************************************************/
/* Copyright (c) 2004                                                    */
/* Daniel Sleator, David Temperley, and John Lafferty                    */
/* Copyright (c) 2013, 2014 Linas Vepstas                                */
/* All rights reserved                                                   */
/*                                                                       */
/* Use of the link grammar parsing system is subject to the terms of the */
/* license set forth in the LICENSE file included with this software.    */
/* This license allows free redistribution and use in source and binary  */
/* forms, with or without modification, subject to certain conditions.   */
/*                                                                       */
/*************************************************************************/

#ifndef _LG_DICT_COMMON_H_
#define  _LG_DICT_COMMON_H_

#include "api-types.h"                  // pp_knowledge
#include "connectors.h"                 // ConTable
#include "dict-defines.h"
#include "dict-structures.h"
#include "memory-pool.h"                // Pool_desc
#include "utilities.h"                  // locale_t

#define EMPTY_CONNECTOR "ZZZ"
#define UNLIMITED_CONNECTORS_WORD ("UNLIMITED-CONNECTORS")
#define LIMITED_CONNECTORS_WORD ("LENGTH-LIMIT-")
#define IS_GENERATION(dict) (dict->category != NULL)

/* If the maximum disjunct cost is yet uninitialized, the value defined in the
 * dictionary (or if not defined then DEFAULT_MAX_DISJUNCT_COST) is used. */
static const double UNINITIALIZED_MAX_DISJUNCT_COST = -10000.0;
static const double DEFAULT_MAX_DISJUNCT_COST = 2.7;
/* We need some of these as literal strings. */
#define LG_DISJUNCT_COST                        "max-disjunct-cost"
#define LG_DICTIONARY_VERSION_NUMBER            "dictionary-version-number"
#define LG_DICTIONARY_LOCALE                    "dictionary-locale"

/* Forward decls */
typedef struct Afdict_class_struct Afdict_class;
typedef struct Regex_node_s Regex_node;

typedef struct X_node_struct X_node;
struct X_node_struct
{
	const char * string;       /* the word itself */
	Exp * exp;
	X_node *next;
	const Gword *word;         /* originating Wordgraph word */
};

/* The regexes are stored as a linked list of the following nodes. */
struct Regex_node_s
{
	const char *name;/* The identifying name of the regex */
	char *pattern;   /* The regular expression pattern */
	bool neg;        /* Negate the match */
	void *re;        /* The compiled regex. void * to avoid
	                    having re library details invading the
	                    rest of the LG system; regex-morph.c
	                    takes care of all matching.
	                  */
	Regex_node *next;
};

struct Afdict_class_struct
{
	size_t mem_elems;     /* number of memory elements allocated */
	size_t length;        /* number of strings */
	char const ** string;
};

#define MAX_TOKEN_LENGTH 250     /* Maximum number of chars in a token */
#define IDIOM_LINK_SZ 16

#ifdef HAVE_SQLITE3
#define IS_DB_DICT(dict) (NULL != dict->db_handle)
#else
#define IS_DB_DICT(dict) false
#endif /* HAVE_SQLITE3 */

/* "#define name value" */
typedef struct
{
	String_id *set;                    /* "define" name set */
	const char **name;
	const char **value;
	unsigned int size;                 /* Allocated value array size */
} define_s;

typedef struct
{
	String_id *set;                    /* Expression tag names */
	const char **name;                 /* Tag name (indexed by tag id) */
	unsigned int num;                  /* Number of tags */
	unsigned int size;                 /* Allocated tag array size */
} expression_tag;

struct Dictionary_s
{
	Dict_node *  root;
	Regex_node * regex_root;
	const char * name;
	const char * lang;
	const char * version;
	const char * locale;    /* Locale name */
	double default_max_disjunct_cost;
	locale_t     lctype;    /* Locale argument for the *_l() functions */
	int          num_entries;
	define_s     define;    /* Name-value definitions */

	bool         use_unknown_word;
	bool         unknown_word_defined;
	bool         left_wall_defined;
	bool         right_wall_defined;
	bool         shuffle_linkages;

	Dialect *dialect;                  /* "4.0.dialect" info */
	expression_tag dialect_tag;        /* Expression dialect tag info */
	expression_tag *macro_tag;         /* Macro tags for expression debug */
	void *cached_dialect;              /* Only for dialect cache validation */

	/* Affixes are used during the tokenization stage. */
	Dictionary      affix_table;
	Afdict_class *  afdict_class;
	bool pre_suf_class_exists;         /* True iff PRE or SUF exists */

	/* Random morphology generator */
	struct anysplit_params * anysplit;

	/* If not null, then use spelling guesser for unknown words */
	void *          spell_checker;     /* spell checker handle */
#ifdef HAVE_SQLITE3
	void *          db_handle;         /* database handle */
#endif

	void (*insert_entry)(Dictionary, Dict_node *, int);
	Dict_node* (*lookup_list)(Dictionary, const char*);
	Dict_node* (*lookup_wild)(Dictionary, const char*);
	void (*free_lookup)(Dictionary, Dict_node*);
	bool (*lookup)(Dictionary, const char*);
	void (*close)(Dictionary);

	pp_knowledge  * base_knowledge;    /* Core post-processing rules */
	pp_knowledge  * hpsg_knowledge;    /* Head-Phrase Structure rules */
	String_set *    string_set;        /* Set of link names in the dictionary */
	Word_file *     word_file_header;
	ConTable        contable;

	Pool_desc  * Exp_pool;

	/* Sentence generation */
	unsigned int num_categories;
	unsigned int num_categories_alloced;
	Category * category;      /* Word lists - indexed by category number */
	bool generate_walls;      /* Generate walls too for wildcard words */

	/* Private data elements that come in play only while the
	 * dictionary is being read, and are not otherwise used.
	 */
	const char    * input;
	const char    * pin;
	bool            recursive_error;
	bool            is_special;
	int             already_got_it; /* For char, but needs to hold EOF */
	int             line_number;
	char            current_idiom[IDIOM_LINK_SZ];
	char            token[MAX_TOKEN_LENGTH];
};

bool is_stem(const char *);
bool is_wall(const char *);
bool is_macro(const char *);

Exp *Exp_create(Pool_desc *);
Exp *Exp_create_dup(Pool_desc *, Exp *);
Exp *make_unary_node(Pool_desc *, Exp *);

bool dictionary_generation_request(const Dictionary);

/* The functions here are intended for use by the tokenizer, only,
 * and pretty much no one else. If you are not the tokenizer, you
 * probably don't need these. */
bool dict_has_word(const Dictionary dict, const char *);
void add_empty_word(Sentence, X_node *);

static inline const char *subscript_mark_str(void)
{
	static const char sm[] = { SUBSCRIPT_MARK, '\0' };
	return sm;
}

/**
 * Get the subscript of the given word.
 * In case of more than one subscript, return the last subscript.
 * The returned value constness is the same as the argument.
 */
static inline char *get_word_subscript(const char *word)
{
	return (char *)strrchr(word, SUBSCRIPT_MARK); /* (char *) for MSVC */
}
#define get_word_subscript(word) _Generic((word), \
	const char * : (const char *)(get_word_subscript)((word)), \
	char *       :               (get_word_subscript)((word)))
#endif /* _LG_DICT_COMMON_H_ */
