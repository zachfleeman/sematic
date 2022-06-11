/*************************************************************************/
/* Copyright (c) 2004                                                    */
/* Daniel Sleator, David Temperley, and John Lafferty                    */
/* Copyright 2018-2020, Amir Plivatsky                                  */
/* All rights reserved                                                   */
/*                                                                       */
/* Use of the link grammar parsing system is subject to the terms of the */
/* license set forth in the LICENSE file included with this software.    */
/* This license allows free redistribution and use in source and binary  */
/* forms, with or without modification, subject to certain conditions.   */
/*                                                                       */
/*************************************************************************/
#include <string.h>

#include "api-structures.h"             // Sentence
#include "connectors.h"
#include "dict-common/dict-api.h"
#include "dict-common/dict-structures.h"
#include "dict-common/regex-morph.h"    // match_regex
#include "disjunct-utils.h"
#include "memory-pool.h"
#include "prepare/build-disjuncts.h"
#include "print/print-util.h"
#include "string-set.h"
#include "tokenize/tok-structures.h"    // XXX TODO provide gword access methods!
#include "tokenize/word-structures.h"
#include "tracon-set.h"
#include "utilities.h"

/* Disjunct API ... */

static char *connector_list_to_expression(const char *connector_list)
{
	dyn_str *e = dyn_str_new();
	for (const char *p = connector_list; *p != '\0'; p++)
	{
		if (*p != ' ')
		{
			dyn_strcat(e, (char []){ *p, '\0' });
			continue;
		}
		if (p[1] != '\0') dyn_strcat(e, " & ");
	}

	return dyn_str_take(e);
}

/**
 * Return the expression of the given disjunct;
 * The caller has to free the returned value.
 */
char * disjunct_expression(Disjunct *d)
{
	char *ls = print_connector_list_str(d->left, "-");
	char *rs = print_connector_list_str(d->right, "+");

	size_t lrs_sz = strlen(ls) + 1 + strlen(rs); /* ls " " rs */
	char *lrs = alloca(lrs_sz + 1);
	size_t n = lg_strlcpy(lrs, ls, lrs_sz);
	if ((ls[0] != '\0') && (rs[0] != '\0'))
	    n += lg_strlcpy(lrs + n, " ", lrs_sz);
	lg_strlcpy(lrs + n, rs, lrs_sz);
	lrs[lrs_sz] = '\0';

	free(ls);
	free(rs);

	return connector_list_to_expression(lrs);
}

/**
 * Return the Category_cost array (NULL terminated) of the given disjunct.
 * It shouldn't be freed by the caller.
 */
const Category_cost * disjunct_categories(Disjunct *d)
{
	if (d->is_category == 0) return NULL;
	return d->category;
}

/**
 * Return a NULL terminated array of pointers to disjuncts which are
 * unused in the current sentence-generation linkage.
 * Note: Only wild-card words are considered (fixed words are currently
 * ignored).
 * The caller has to free the returned value.
 */
Disjunct ** sentence_unused_disjuncts(Sentence sent)
{
	if ((sent == NULL) || (sent->disjunct_used == NULL)) return NULL;

	unsigned int n = 0;
	for (unsigned int i = 0; i < sent->wildcard_word_num_disjuncts; i++)
	{
		if (!sent->disjunct_used[i]) n++;
	}
	const size_t unused_d_sz = sizeof(Disjunct *) * (n + 1); /* 1 for NULL */
	Disjunct **unused_d = malloc(unused_d_sz);

	n = 0;
	for (unsigned int i = 0; i < sent->wildcard_word_num_disjuncts; i++)
	{
		if (!sent->disjunct_used[i])
			unused_d[n++] = &((Disjunct *)sent->wildcard_word_dc_memblock)[i];
	}
	unused_d[n] = NULL;

	return unused_d;
}

/* Disjunct utilities ... */

#define D_DISJ 5                        /* Verbosity level for this file. */

/**
 * free_disjuncts() -- free the list of disjuncts pointed to by c
 * (does not free any strings)
 */
void free_disjuncts(Disjunct *c)
{
	Disjunct *c1;
	for (;c != NULL; c = c1) {
		c1 = c->next;
		free_connectors(c->left);
		free_connectors(c->right);
		xfree((char *)c, sizeof(Disjunct));
	}
}

void free_categories_from_disjunct_array(Disjunct *dbase,
                                         unsigned int num_disjuncts)
{
	for (Disjunct *d = dbase; d < &dbase[num_disjuncts]; d++)
	{
		if (d->is_category != 0)
			free(d->category);
	}
}

void free_categories(Sentence sent)
{
	if (NULL != sent->dc_memblock)
	{
		free_categories_from_disjunct_array(sent->dc_memblock,
		                                    sent->num_disjuncts);
	}
	else
	{
		for (WordIdx w = 0; w < sent->length; w++)
		{
			for (Disjunct *d = sent->word[w].d; d != NULL; d = d->next)
			{
				if (d->is_category != 0)
					free(d->category);
			}
		}
	}
}

void free_sentence_disjuncts(Sentence sent, bool category_too)
{
	if (NULL != sent->dc_memblock)
	{
		if (category_too) free_categories(sent);
		free(sent->dc_memblock);
		sent->dc_memblock = NULL;
	}
	else if (NULL != sent->Disjunct_pool)
	{
		pool_delete(sent->Disjunct_pool);
		pool_delete(sent->Connector_pool);
		sent->Disjunct_pool = NULL;
	}
}

/**
 * Destructively catenates the two disjunct lists d1 followed by d2.
 * Doesn't change the contents of the disjuncts.
 * Traverses the first list, but not the second.
 */
Disjunct * catenate_disjuncts(Disjunct *d1, Disjunct *d2)
{
	Disjunct * dis = d1;

	if (d1 == NULL) return d2;
	if (d2 == NULL) return d1;
	while (dis->next != NULL) dis = dis->next;
	dis->next = d2;
	return d1;
}

/** Returns the number of disjuncts in the list pointed to by d */
unsigned int count_disjuncts(Disjunct * d)
{
	unsigned int count = 0;
	for (; d != NULL; d = d->next)
	{
		count++;
	}
	return count;
}

/** Returns the number of connectors in the sentence. */
static unsigned int count_connectors(Sentence sent)
{
	unsigned int ccnt = 0;

	for (WordIdx w = 0; w < sent->length; w++)
	{
		for (Disjunct *d = sent->word[w].d; d != NULL; d = d->next)
		{
			for (Connector *c = d->left; c != NULL; c = c->next) ccnt++;
			for (Connector *c = d->right; c !=NULL; c = c->next) ccnt++;
		}
	}

	return ccnt;
}
/* ============================================================= */

typedef struct disjunct_dup_table_s disjunct_dup_table;
struct disjunct_dup_table_s
{
	size_t dup_table_size;
	Disjunct *dup_table[];
};

/**
 * This is a hash function for disjuncts
 *
 * This is the old version that doesn't check for domination, just
 * equality.
 */
static inline unsigned int old_hash_disjunct(disjunct_dup_table *dt,
                                             Disjunct * d, bool string_too)
{
	unsigned int i;
	i = 0;
	for (Connector *e = d->left; e != NULL; e = e->next) {
		i = (41 * (i + e->desc->uc_num)) + (unsigned int)e->desc->lc_letters + 7;
	}
	for (Connector *e = d->right; e != NULL; e = e->next) {
		i = (41 * (i + e->desc->uc_num)) + (unsigned int)e->desc->lc_letters + 7;
	}
	if (string_too)
		i += string_hash(d->word_string);
	i += (i>>10);

	d->dup_hash = i;
	return (i & (dt->dup_table_size-1));
}

/**
 * The connectors must be exactly equal.
 */
static bool connectors_equal_prune(Connector *c1, Connector *c2)
{
	return c1->desc == c2->desc && (c1->multi == c2->multi);
}

/** returns TRUE if the disjuncts are exactly the same */
static bool disjuncts_equal(Disjunct * d1, Disjunct * d2, bool ignore_string)
{
	Connector *e1, *e2;

	e1 = d1->left;
	e2 = d2->left;
	while ((e1 != NULL) && (e2 != NULL)) {
		if (!connectors_equal_prune(e1, e2)) return false;
		e1 = e1->next;
		e2 = e2->next;
	}
	if ((e1 != NULL) || (e2 != NULL)) return false;

	e1 = d1->right;
	e2 = d2->right;
	while ((e1 != NULL) && (e2 != NULL)) {
		if (!connectors_equal_prune(e1, e2)) return false;
		e1 = e1->next;
		e2 = e2->next;
	}
	if ((e1 != NULL) || (e2 != NULL)) return false;

	if (ignore_string) return true;

	/* Save CPU time by comparing this last, since this will
	 * almost always be true. Rarely, the strings are not from
	 * the same string_set and hence the 2-step comparison. */
	if (d1->word_string == d2->word_string) return true;
	return (strcmp(d1->word_string, d2->word_string) == 0);
}

#if 0
int de_fp = 0;
int de_total = 0;
static void disjuncts_equal_stat(void)
{
		fprintf(stderr, "disjuncts_equal FP %d/%d\n", de_fp, de_total);
}

static bool disjuncts_equal(Disjunct * d1, Disjunct * d2, bool ignore_string)
{
	if (de_total == 0) atexit(disjuncts_equal_stat);
	de_total++;

	bool rc = disjuncts_equal1(d1, d2, bool ignore_string);
	if (!rc) de_fp++;

	return rc;
}
#endif

static disjunct_dup_table * disjunct_dup_table_new(size_t sz)
{
	disjunct_dup_table *dt;

	dt = malloc(sz * sizeof(Disjunct *) + sizeof(disjunct_dup_table));
	dt->dup_table_size = sz;

	memset(dt->dup_table, 0, sz * sizeof(Disjunct *));

	return dt;
}

static void disjunct_dup_table_delete(disjunct_dup_table *dt)
{
	free(dt);
}

#ifdef DEBUG
GNUC_UNUSED static int gword_set_len(const gword_set *gl)
{
	int len = 0;
	for (; NULL != gl; gl = gl->next) len++;
	return len;
}
#endif

/**
 * Return a new gword_set element, initialized from the given element.
 * @param old_e Existing element.
 */
static gword_set *gword_set_element_new(gword_set *old_e)
{
	gword_set *new_e = malloc(sizeof(gword_set));
	*new_e = (gword_set){0};

	new_e->o_gword = old_e->o_gword;
	gword_set *chain_next = old_e->chain_next;
	old_e->chain_next = new_e;
	new_e->chain_next = chain_next;

	return new_e;
}

/**
 * Add an element to existing gword_set. Uniqueness is assumed.
 * @return A new set with the element.
 */
static gword_set *gword_set_add(gword_set *gset, gword_set *ge)
{
	gword_set *n = gword_set_element_new(ge);
	n->next = gset;
	gset = n;

	return gset;
}

/**
 * Combine the given gword sets.
 * The gword sets are not modified.
 * This function is used for adding the gword pointers of an eliminated
 * disjunct to the ones of the kept disjuncts, with no duplicates.
 *
 * @param kept gword_set of the kept disjunct.
 * @param eliminated gword_set of the eliminated disjunct.
 * @return Use copy-on-write semantics - the gword_set of the kept disjunct
 * just gets returned if there is nothing to add to it. Else - a new gword
 * set is returned.
 */
static gword_set *gword_set_union(gword_set *kept, gword_set *eliminated)
{
	/* Preserve the gword pointers of the eliminated disjunct if different. */
	gword_set *preserved_set = NULL;
	for (gword_set *e = eliminated; NULL != e; e = e->next)
	{
		gword_set *k;

		/* Ensure uniqueness. */
		for (k = kept; NULL != k; k = k->next)
			if (e->o_gword == k->o_gword) break;
		if (NULL != k) continue;

		preserved_set = gword_set_add(preserved_set, e);
	}

	if (preserved_set)
	{
		/* Preserve the originating gword pointers of the remaining disjunct. */
		for (gword_set *k = kept; NULL != k; k = k->next)
			preserved_set = gword_set_add(preserved_set, k);
		kept = preserved_set;
	}

	return kept;
}

/**
 * Takes the list of disjuncts pointed to by d, eliminates all
 * duplicates, and returns a pointer to a new list.
 */
Disjunct *eliminate_duplicate_disjuncts(Disjunct *dw, bool multi_string)
{
	unsigned int count = 0;
	disjunct_dup_table *dt;
	/* This initialization is unneeded because the first disjunct is never
	 * eliminated. However, omitting it generates "uninitialized" compiler
	 * warning. Setting it to NULL generates clang-analyzer error on
	 * possible NULL dereference. */
	Disjunct *prev = dw;

	dt = disjunct_dup_table_new(next_power_of_two_up(2 * count_disjuncts(dw)));

	for (Disjunct *d = dw; d != NULL; d = d->next)
	{
		Disjunct *dx;
		unsigned int h = old_hash_disjunct(dt, d, /*string_too*/!multi_string);

		for (dx = dt->dup_table[h]; dx != NULL; dx = dx->dup_table_next)
		{
			if (d->dup_hash != dx->dup_hash) continue;
			if (disjuncts_equal(dx, d, multi_string)) break;
		}

		if (dx != NULL)
		{
			/* Discard the current disjunct. */

			if (multi_string)
			{
				if (dx->num_categories == dx->num_categories_alloced - 1)
				{
					dx->num_categories_alloced *= 2;
					dx->category = realloc(dx->category,
					   sizeof(*(dx->category)) * dx->num_categories_alloced);
				}
				dassert((d->category[0].num > 0) && (d->category[0].num < 64*1024),
				        "Insane category %u", d->category[0].num);
				dx->category[dx->num_categories].num = d->category[0].num;
				dx->category[dx->num_categories].cost = d->cost;
				dx->num_categories++;
				dx->category[dx->num_categories].num = 0; /* API array terminator.*/
			}
			else
			{
				if (d->cost < dx->cost) dx->cost = d->cost;
				dx->originating_gword =
					gword_set_union(dx->originating_gword, d->originating_gword);
			}

			count++;
			prev->next = d->next;
			if (d->is_category != 0)
			{
				free(d->category);
				d->is_category = 0; /* Save free() call on sentence delete. */
			}
		}
		else
		{
			d->dup_table_next = dt->dup_table[h];
			dt->dup_table[h] = d;
			prev = d;
		}
	}

	lgdebug(+D_DISJ+(0==count)*1024, "w%zu: Killed %u duplicates%s\n",
	        dw->originating_gword->o_gword->sent_wordidx, count,
	        multi_string ? " (different word-strings)" : "");

	disjunct_dup_table_delete(dt);
	return dw;
}

/* ============================================================= */

/* Return the stringified disjunct.
 * Be sure to free the string upon return.
 */

static void prt_con(Connector *c, dyn_str * p, char dir)
{
	if (NULL == c) return;
	prt_con (c->next, p, dir);

	if (c->multi)
	{
		append_string(p, "@%s%c ", connector_string(c), dir);
	}
	else
	{
		append_string(p, "%s%c ", connector_string(c), dir);
	}
}

char *print_one_disjunct_str(const Disjunct *dj)
{
	dyn_str *p = dyn_str_new();

	prt_con(dj->left, p, '-');
	prt_con(dj->right, p, '+');

	return dyn_str_take(p);
}

/* ============================================================= */

/**
 * returns the number of connectors in the left lists of the disjuncts.
 */
int left_connector_count(Disjunct * d)
{
	int i=0;
	for (;d!=NULL; d=d->next) {
		for (Connector *c = d->left; c!=NULL; c = c->next) i++;
	}
	return i;
}

int right_connector_count(Disjunct * d)
{
	int i=0;
	for (;d!=NULL; d=d->next) {
	  for (Connector *c = d->right; c!=NULL; c = c->next) i++;
	}
	return i;
}

/** Returns the number of disjuncts and connectors in the sentence. */
void count_disjuncts_and_connectors(Sentence sent, unsigned int *dca,
                                    unsigned int *cca)
{
	unsigned int ccnt = 0, dcnt = 0;

	for (WordIdx w = 0; w < sent->length; w++)
	{
		for (Disjunct *d = sent->word[w].d; d != NULL; d = d->next)
		{
			dcnt++;
			for (Connector *c = d->left; c != NULL; c = c->next) ccnt++;
			for (Connector *c = d->right; c !=NULL; c = c->next) ccnt++;
		}
	}

	*cca = ccnt;
	*dca = dcnt;
}

/* ============= Connector encoding, sharing and packing ============= */

/*
 * sentence_pack() copies the disjuncts and connectors to a contiguous
 * memory. This facilitate a better memory caching for long sentences.
 *
 * In addition, it shares the memory of identical trailing connector
 * sequences, aka "tracons". Tracons are considered identical if they
 * belong to the same Gword (or same word for the pruning step) and
 * contain identical connectors in the same order (with one exception:
 * shallow connectors must have the same nearest_word as tracon leading
 * deep connectors). Connectors are considered identical if they have
 * the same string representation (including "multi" and the direction
 * mark) with an additional requirement if the packing is done for the
 * pruning step - shallow and deep connectors are then not considered
 * identical. In both cases the exception regarding shallow connectors
 * is because shallow connectors can match any connector, while deep
 * connectors can match only shallow connectors. Note: For efficiency,
 * the actual connector string representation is not used for connector
 * comparison.
 *
 * For the parsing step, identical tracons are assigned a unique tracon
 * ID, which is kept in their first connector tracon_id field. The rest of
 * their connectors also have tracon IDs, which belong to tracons starting
 * with that connectors. The tracon_id is not used for pruning.
 *
 * For the pruning step, more things are done:
 * Additional data structure - a tracon list - is constructed, which
 * includes a tracon table and per-word prune table sizes. These data
 * structure consists of 2 identical parts - one for each tracon
 * direction (left/right). The tracon table is indexed by (tracon_id -
 * 1), and it indexes the connectors memory block (it doesn't use
 * pointers in order to save memory on 64-bit CPUs because it may
 * contain in the order of 100K entries for very long sentences).
 * Also, a refcount field is set for each tracon to tell how many
 * tracons are memory-shared at that connector address.
 *
 * Tracons are used differently in the pruning and parsing steps.
 *
 * Power Pruning:
 * The first connector of each tracon is inserted into the power table,
 * along with its reference count. When a connector cannot be matched,
 * this means that all the disjuncts that contain its tracon also cannot
 * be matched. It is then marked as bad (by nearest_word=BAD_WORD) and
 * due to the tracon memory sharing all the connectors that share the same
 * memory are marked simultaneously, and thus are detected when the next
 * disjuncts are examined without a need to further process them.
 * This drastically reduces the "power_cost" and saves much processing.
 * Setting the nearest_word field is hence done only once per tracon on
 * each pass. The pass_number field is used to detect already-processed
 * good tracons - they are assigned the pass number so each tracon is
 * processed only once per pass. The connector refcount field is used to
 * discard connectors from the power table when all the disjuncts that
 * contain them are discarded.
 *
 * PP pruning:
 * Here too only the first connector in each tracon needs to be
 * examined. Marking a connector with BAD_WORD simultaneously leaves
 * a mark in the corresponding connector in the cms table and in all
 * the disjuncts that share it.
 *
 * Parsing:
 * Originally, the classic parser memoized the number of possible
 * linkages per a given connector-pair using connector addresses. Since
 * an exhaustive search is done, such an approach has two main problems
 * for long sentences:
 * 1. A very big count hash table (Table_connector in count.c) is used
 * due to the huge number of connectors (100Ks) in long sentences, a
 * thing that causes a severe CPU cache trash (to the degree that
 * absolutely most of the memory accesses are L3 misses).
 * 2. Repeated linkage detailed calculation for what seems identical
 * connectors. A hint for the tracon idea was the use of 0 hash values
 * for NULL connectors, which is the same for all the disjuncts of the
 * same word (they can be considered a private case of a tracon - a
 * "null tracon").
 *
 * The idea that is implemented here is based on the fact that the
 * number of linkages between the same words using any of their
 * connector-pair endpoints is governed only by these connectors and the
 * connectors after them (since cross links are not permitted). Using
 * tracon IDs as the hash keys allows to share the memoizing table
 * counts between connectors that start the same tracons. As a
 * result, long sentences have significantly less different connector
 * hash values than their total number of connectors.
 *
 * In order to save the need to cache and check the endpoint word
 * numbers the tracon IDs should not be shared between words. They also
 * should not be shared between alternatives since connectors that belong
 * to disjuncts of different alternatives may have different linkage
 * counts because some alternatives-connectivity checks (to the middle
 * disjunct) are done in the fast-matcher. These restrictions are
 * implemented by using a different tracon ID per Gword (FIXME - this is
 * more strict then needed - a different tracon ID per alternative would
 * suffice).
 * The tracon memory sharing is currently not directly used in the
 * parsing algo besides reducing the needed CPU cache by a large factor.
 *
 * Algo of generating tracon Ids, shared tracons and the tracon list:
 * The string-set code has been adapted (see tracon-set.c) to hash
 * tracons. The tracon-set hash table slots are Connector pointers which
 * point to the memory block of the sentence connectors. When a tracon
 * is not found in the hash table, a new tracon ID is assigned to it,
 * and the tracon is copied to the said connector memory block. However,
 * if it is found, its address is used instead of copying the
 * connectors, thus sharing its memory with identical tracons. The
 * tracon-set hash table is cleared after each word (for pruning tracons)
 * or Gword (for parsing tracons), thus ensuring that the tracons IDs are
 * not shared between words (or Gwords).
 *
 * Some tracon features:
 * - Each connector starts some tracon.
 * - Connectors of identical tracons share their memory.
 *
 * Jets:
 * A jet is a (whole) ordered set of connectors all pointing in the same
 * direction (left, or right). Every disjunct can be split into two jets;
 * that is, a disjunct is a pair of jets, and so each word consists of a
 * collection of pairs of jets. The first connector in a jet called
 * a "shallow" connector. Connectors that are not shallow are deep.
 * See the comments in prune.c for their connection properties.
 * A jet is also a tracon.
 *
 * Note: This comment is referred-to in disjunct-utils.h, so changes
 * here may need to be reflected in the comments there too.
 */

static void tlsz_check(Tracon_list *tl, unsigned int index, int dir)
{

	if (index >= tl->table_size[dir])
	{
		size_t new_id_table_size = (0 == tl->table_size[dir]) ?
			index : tl->table_size[dir] * 2;
		size_t new_bytes = new_id_table_size * sizeof(uint32_t *);

		tl->table[dir] = realloc(tl->table[dir], new_bytes);
		tl->table_size[dir] = new_id_table_size;
	}
}

/**
 * Pack the connectors in an array; memory-share and enumerate tracons.
 */
static Connector *pack_connectors(Tracon_sharing *ts, Connector *origc, int dir,
                                  int w)
{
	if (NULL == origc) return NULL;

	Connector head;
	Connector *prevc = &head;
	Connector *newc = &head;
	Connector *lcblock = ts->cblock;     /* For convenience. */
	Tracon_list *tl = ts->tracon_list;   /* If non-NULL - encode for pruning. */

	for (Connector *o = origc; NULL != o;  o = o->next)
	{
		newc = NULL;

		if (NULL != ts->csid[dir])
		{
			/* Encoding is used - share tracons. */
			Connector **tracon = tracon_set_add(o, ts->csid[dir]);

			if (NULL == *tracon)
			{
				/* The first time we encounter this tracon. */
				*tracon = lcblock; /* Save its future location in the tracon_set. */

				if (NULL != tl)
				{
					tlsz_check(tl, tl->entries[dir], dir);
					uint32_t cblock_index = (uint32_t)(lcblock - ts->cblock_base);
					tl->table[dir][tl->entries[dir]] = cblock_index;
					tl->entries[dir]++;
				}
			}
			else
			{
				newc = *tracon;
				if (!ts->is_pruning)
				{
					if ((o->nearest_word != newc->nearest_word) ||
					    (o->farthest_word != newc->farthest_word))
					{
						/* This is a rare case in which a shallow and deep
						 * connectors don't have the same nearest_word, because
						 * a shallow connector may match a deep connector
						 * earlier. Because the nearest word is different, we
						 * cannot share it. (Such shallow and deep tracons could
						 * be shared separately, but because this is a rare
						 * event there is no benefit to do that.)
						 * Note:
						 * In case the parsing ever depends on other Connector
						 * fields, there will be a need to add a check for them
						 * here.
						 * Update: farthest_word added. */
						newc = NULL; /* Don't share it. */
					}
				}
			}
		}

		if (newc == NULL)
		{
			/* No sharing yet. */
			newc = lcblock++;
			*newc = *o;

			if (ts->is_pruning)
			{
				/* Tracon seen for first time - initialize for the pruning stage. */
				newc->refcount = 1;  /* No sharing yet. */
				if (ts->uc_seen[dir][connector_uc_num(newc)] != w)
				{
					ts->uc_seen[dir][connector_uc_num(newc)] = w;
					ts->num_cnctrs_per_word[dir][w]++;
				}
			}
			else
			{
				/* For the parsing stage we need a unique ID. */
				newc->tracon_id = ts->next_id[dir]++;
			}
		}
		else
		{
			if (NULL != tl)
			{
				for (Connector *n = newc; NULL != n; n = n->next)
					n->refcount++;
			}
			prevc->next = newc;

			/* Just shared a tracon, nothing more to do. */
			ts->cblock = lcblock;
			return head.next;
		}

		prevc->next = newc;
		prevc = newc;
	}
	newc->next = NULL;

	ts->cblock = lcblock;
	return head.next;
}

static Disjunct *pack_disjunct(Tracon_sharing *ts, Disjunct *d, int w)
{
	Disjunct *newd;
	uintptr_t token = (uintptr_t)w;

	newd = (ts->dblock)++;
	newd->word_string = d->word_string;
	newd->cost = d->cost;
	newd->is_category = d->is_category;
	newd->originating_gword = d->originating_gword;
	newd->ordinal = d->ordinal;

	if (NULL == ts->tracon_list)
		 token = (uintptr_t)d->originating_gword;

	if ((token != ts->last_token) && (NULL != ts->csid[0]))
	{
		ts->last_token = token;
		//printf("Token %ld\n", token);
		tracon_set_reset(ts->csid[0]);
		tracon_set_reset(ts->csid[1]);
	}
	newd->left = pack_connectors(ts, d->left, 0, w);
	newd->right = pack_connectors(ts, d->right, 1,  w);

	return newd;
}

/**
 * Pack the given disjunct chain in a contiguous memory block.
 * If the disjunct is NULL, return NULL.
 */
static Disjunct *pack_disjuncts(Sentence sent, Tracon_sharing *ts,
                                Disjunct *origd, int w)
{
	Disjunct head;
	Disjunct *prevd = &head;

	for (Disjunct *d = origd; NULL != d; d = d->next)
	{
		prevd->next = pack_disjunct(ts, d, w);
		prevd = prevd->next;
	}
	prevd->next = NULL;

	return head.next;
}

#define TLSZ 8192         /* Initial size of the tracon list table */

/* Reserved tracon ID space for NULL connectors (zero-length tracons).
 * Currently, tracons are unique per word. So this is actually the max.
 * number of words in a sentence rounded up to a power of 2.
 * FIXME: Derive it from MAX_SENTENCE. */
#define WORD_OFFSET 256

/** Create a context descriptor for disjuncts & connector memory "packing".
 *   Allocate a memory block for all the disjuncts & connectors.
 *   The current Connector struct size is 32 bytes, and the intention is
 *   to keep it with a power-of-2 size. The idea is to put an integral
 *   number of connectors in each cache line (assumed to be >= Connector
 *   struct size, e.g. 64 bytes), so one connector will not need 2 cache
 *   lines.
 *
 *   The current Disjunct struct size is 64 bytes, and the intention is
 *   to keep it at this size for performance reasons.
 *
 *   The allocated memory block includes 2 sections, in that order:
 *   1. A block for disjuncts.
 *   2. A block of connectors.
 *
 *   If encoding is done for the pruning step, allocate tracon list
 *   stuff too. In that case also call tracon_set_shallow() so tracons
 *   starting with a shallow connector will be considered different than
 *   similar ones starting with a deep connector.
 *
 * Note:
 * In order to save overhead, sentences shorter than
 * sent->min_len_encoding don't undergo encoding - only packing.
 * This can also be used for library tests that totally bypass the use of
 * connector encoding (to validate that the tracon_id/sharing/refcount
 * implementation didn't introduce bugs in the pruning and parsing steps).
 * E.g. when using link-parser:
 * - To entirely disable connector encoding:
 * link-parser -test=min-len-encoding:254
 * - To use connector encoding even for short sentences:
 * link-parser -test=min-len-encoding:0
 * Any different result (e.g. number of discarded disjuncts in the pruning
 * step or different parsing results) indicates a bug.
 *
 * @param is_pruning TRUE if invoked for pruning, FALSE if invoked for parsing.
 * @return The said context descriptor.
 */
static Tracon_sharing *pack_sentence_init(Sentence sent, bool is_pruning)
{
	unsigned int dcnt = 0, ccnt = 0;
	count_disjuncts_and_connectors(sent, &dcnt, &ccnt);

	size_t dsize = dcnt * sizeof(Disjunct);
	if (sizeof(Disjunct) != 64)
		dsize = ALIGN(dsize, sizeof(Connector));
	size_t csize = ccnt * sizeof(Connector);
	size_t memblock_sz = dsize + csize;
	void *memblock = malloc(memblock_sz);
	Disjunct *dblock = memblock;
	Connector *cblock = (Connector *)((char *)memblock + dsize);

	Tracon_sharing *ts = malloc(sizeof(Tracon_sharing));
	memset(ts, 0, sizeof(Tracon_sharing));

	ts->memblock = memblock;
	ts->memblock_sz = memblock_sz;
	ts->cblock_base = cblock;
	ts->cblock = cblock;
	ts->dblock = dblock;
	ts->num_connectors = ccnt;
	ts->num_disjuncts = dcnt;
	ts->word_offset = is_pruning ? 1 : WORD_OFFSET;
	ts->is_pruning = is_pruning;
	ts->next_id[0] = ts->next_id[1] = ts->word_offset;
	ts->last_token = (uintptr_t)-1;

	if (is_pruning)
	{
		/* Allocate and initialize memory for finding the number of
		 * different uppercase connector parts per direction / word, for
		 * sizing the pruning power table. */
		unsigned int **ncu = ts->num_cnctrs_per_word;
		ncu[0] = malloc(2 * sent->length * sizeof(**ncu));
		ncu[1] = ncu[0] + sent->length;
		memset(ncu[0], 0, 2 * sent->length * sizeof(**ncu));

		size_t uc_num = sent->dict->contable.num_uc;
		ts->uc_seen[0] = malloc(2 * uc_num * sizeof(**ts->uc_seen));
		ts->uc_seen[1] = ts->uc_seen[0] + uc_num;
		/* Initialize w/an invalid word number in a hopefully (**uc_seen)
		 * size independent manner.
		 * Note that (unsigned char)-1 is currently MAX_SENTENCE+1. */
		memset(ts->uc_seen[0], -1, 2 * uc_num * sizeof(**ts->uc_seen));
	}

	/* Encode connectors only for long-enough sentences. */
	if (sent->length >= sent->min_len_encoding)
	{
		ts->csid[0] = tracon_set_create();
		ts->csid[1] = tracon_set_create();

		if (is_pruning)
		{
			Tracon_list *tl;

			tl = ts->tracon_list = malloc(sizeof(Tracon_list));
			memset(tl, 0, sizeof(Tracon_list));
			for (int dir = 0; dir < 2; dir++)
			{

				tracon_set_shallow(true, ts->csid[dir]);
				tlsz_check(tl, TLSZ, dir); /* Allocate table. */
			}
		}
	}

	if (!is_pruning && (ts->memblock != sent->dc_memblock))
	{
		/* The disjunct & connector content is stored in dc_memblock.
		 * It will be freed at sentence_delete(). */
		if (sent->dc_memblock) free(sent->dc_memblock);
		sent->dc_memblock = ts->memblock;
		sent->num_disjuncts = ts->num_disjuncts;
	}

	return ts;
}

void free_tracon_sharing(Tracon_sharing *ts)
{
	if (NULL == ts) return;

	for (int dir = 0; dir < 2; dir++)
	{
		if (NULL != ts->tracon_list)
			free(ts->tracon_list->table[dir]);

		if (NULL != ts->csid[dir])
		{
			tracon_set_delete(ts->csid[dir]);
			ts->csid[dir] = NULL;
		}

	}

	free(ts->uc_seen[0]);
	free(ts->num_cnctrs_per_word[0]);

	if (NULL != ts->d) free(ts->d);
	free(ts->tracon_list);
	ts->tracon_list = NULL;

	free(ts);
}

/**
 * Pack all disjunct and connectors into one big memory block, share
 * tracon memory and generate tracon IDs (for parsing) or tracon lists
 * with reference count (for pruning). Aka "connector encoding".
 *
 * The disjunct and connectors packing in a contiguous memory facilitate a
 * better memory caching for long sentences (a performance gain of a few
 * percents in the initial implementation, in which this was the sole
 * purpose of this packing.) In addition, tracon memory sharing
 * drastically reduces the memory used for connectors.
 *
 * The tracon IDs (if invoked for the parsing step) or tracon lists (if
 * invoked for pruning step) allow for a huge performance boost at these
 * steps.
 */
static Tracon_sharing *pack_sentence(Sentence sent, bool is_pruning)
{
	Tracon_sharing *ts = pack_sentence_init(sent, is_pruning);

	for (WordIdx w = 0; w < sent->length; w++)
	{
		sent->word[w].d = pack_disjuncts(sent, ts, sent->word[w].d, w);
	}

	return ts;
}

/**
 * Pack the sentence for pruning.
 * @return New tracon sharing descriptor.
 */
Tracon_sharing *pack_sentence_for_pruning(Sentence sent)
{
	unsigned int ccnt_before = 0;
	if (verbosity_level(D_DISJ)) ccnt_before = count_connectors(sent);

	Tracon_sharing *ts = pack_sentence(sent, true);

	if (NULL == ts->csid[0])
	{
		lgdebug(D_DISJ, "Debug: Encode for pruning (len %zu): None\n",
		        sent->length);
	}
	else
	{
		lgdebug(D_DISJ, "Debug: Encode for pruning (len %zu): "
		        "tracon_id %zu (%zu+,%zu-), shared connectors %d\n",
		        sent->length,
		        ts->tracon_list->entries[0]+ts->tracon_list->entries[1],
		        ts->tracon_list->entries[0], ts->tracon_list->entries[1],
				  (int)(&ts->cblock_base[ccnt_before] - ts->cblock));
	}

	return ts;
}

/**
 * Pack the sentence for parsing.
 * @return New tracon sharing descriptor.
 */
Tracon_sharing *pack_sentence_for_parsing(Sentence sent)
{
	unsigned int ccnt_before = 0;
	if (verbosity_level(D_DISJ)) ccnt_before = count_connectors(sent);

	Tracon_sharing *ts = pack_sentence(sent, false);

	if (verbosity_level(D_SPEC+2))
	{
		printf("pack_sentence_for_parsing (null_count %u):\n", sent->null_count);
		print_all_disjuncts(sent);
	}

	if (NULL == ts->csid[0])
	{
		lgdebug(D_DISJ, "Debug: Encode for parsing (len %zu): None\n",
		        sent->length);
	}
	else
	{
		lgdebug(D_DISJ, "Debug: Encode for parsing (len %zu): "
		        "tracon_id %d (%d+,%d-), shared connectors %d\n",
		        sent->length,
		        (ts->next_id[0]-ts->word_offset)+(ts->next_id[1]-ts->word_offset),
		        ts->next_id[0]-ts->word_offset, ts->next_id[1]-ts->word_offset,
		        (int)(&ts->cblock_base[ccnt_before] - ts->cblock));
	}

	return ts;
}

/* ============ Save and restore sentence disjuncts ============ */
void *save_disjuncts(Sentence sent, Tracon_sharing *ts)
{
	void *saved_memblock = malloc(ts->memblock_sz);
	memcpy(saved_memblock, ts->memblock, ts->memblock_sz);

	if (NULL == ts->d)
		ts->d = malloc(sent->length * sizeof(Disjunct *));
	for (WordIdx w = 0; w < sent->length; w++)
		ts->d[w] = sent->word[w].d;

	return saved_memblock;
}

void restore_disjuncts(Sentence sent, void *saved_memblock, Tracon_sharing *ts)
{
	if (NULL == saved_memblock) return;

	for (WordIdx w = 0; w < sent->length; w++)
		sent->word[w].d = ts->d[w];

	memcpy(ts->memblock, saved_memblock, ts->memblock_sz);
}
