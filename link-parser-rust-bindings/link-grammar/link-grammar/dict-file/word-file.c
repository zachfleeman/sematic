/***************************************************************************/
/* Copyright (c) 2004                                                      */
/* Daniel Sleator, David Temperley, and John Lafferty                      */
/* All rights reserved                                                     */
/*                                                                         */
/* Use of the link grammar parsing system is subject to the terms of the   */
/* license set forth in the LICENSE file included with this software.      */
/* This license allows free redistribution and use in source and binary    */
/* forms, with or without modification, subject to certain conditions.     */
/*                                                                         */
/***************************************************************************/

#include "error.h"
#include "dict-common/dict-common.h"
#include "dict-common/dict-defines.h" // for MAX_WORD
#include "dict-common/dict-utils.h" // for patch_subscript()
#include "dict-common/file-utils.h"
#include "string-set.h"
#include "read-dict.h"
#include "word-file.h"

/**
 * Reads in one word from the file, allocates space for it,
 * and returns it.
 *
 * In case of an error, return a null string (cannot be a valid word).
 */
static const char * get_a_word(Dictionary dict, FILE * fp)
{
	char word[MAX_WORD+4]; /* allow for 4-byte wide chars */
	const char * s;
	int c, j;

	do {
		c = fgetc(fp);
	} while ((c != EOF) && lg_isspace(c));
	if (c == EOF) return NULL;

	for (j=0; (j <= MAX_WORD-1) && (!lg_isspace(c)) && (c != EOF); j++)
	{
		word[j] = c;
		c = fgetc(fp);
	}

	if (j >= MAX_WORD) {
		word[MAX_WORD] = '\0';
		prt_error("The dictionary contains a word that is too long: %s\n", word);
		return ""; /* error indication */
	}
	word[j] = '\0';
	patch_subscript(word);
	s = string_set_add(word, dict->string_set);
	return s;
}

/**
 *
 * (1) opens the word file and adds it to the word file list
 * (2) reads in the words
 * (3) puts each word in a Dict_node
 * (4) links these together by their left pointers at the
 *     front of the list pointed to by dn
 * (5) returns a pointer to the first of this list
 */
Dict_node * read_word_file(Dictionary dict, Dict_node * dn, char * filename)
{
	Word_file * wf;
	FILE * fp;
	const char * s;

	filename += 1; /* get rid of leading '/' */

	if ((fp = dictopen(filename, "r")) == NULL) {
		return NULL;
	}

	wf = malloc(sizeof (Word_file));
	wf->file = string_set_add(filename, dict->string_set);
	wf->next = dict->word_file_header;
	dict->word_file_header = wf;

	while ((s = get_a_word(dict, fp)) != NULL) {
		if ('\0' == s[0]) /* returned error indication */
		{
			fclose(fp);
			free_insert_list(dn);
			return NULL;
		}
		Dict_node * dn_new = malloc(sizeof(Dict_node));
		dn_new->left = dn;
		dn = dn_new;
		dn->string = s;
		dn->file = wf->file;
	}
	fclose(fp);
	return dn;
}

void free_Word_file(Word_file * wf)
{
	Word_file *wf1;

	for (;wf != NULL; wf = wf1) {
		wf1 = wf->next;
		free(wf);
	}
}

