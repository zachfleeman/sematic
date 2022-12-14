/*************************************************************************/
/* Copyright (c) 2014,2018 Linas Vepstas                                 */
/* All rights reserved                                                   */
/*                                                                       */
/* Use of the link grammar parsing system is subject to the terms of the */
/* license set forth in the LICENSE file included with this software.    */
/* This license allows free redistribution and use in source and binary  */
/* forms, with or without modification, subject to certain conditions.   */
/*                                                                       */
/*************************************************************************/

// This implements a very simple-minded multi-threaded unit test.
// All it does is to make sure the system doesn't crash e.g. due to
// memory allocation conflicts.

#include <thread>
#include <vector>

#include <locale.h>
#include <stdio.h>
#include <stdlib.h>
#include "link-grammar/link-includes.h"

static void parse_one_sent(const char *sent_str)
{
	Parse_Options opts = parse_options_create();
	// Dictionary dict = dictionary_create_lang("ru");
	Dictionary dict = dictionary_create_lang("en");
	if (!dict) {
		fprintf (stderr, "Fatal error: Unable to open the dictionary\n");
		exit(1);
	}

	Sentence sent = sentence_create(sent_str, dict);
	if (!sent) {
		fprintf (stderr, "Fatal error: Unable to create parser\n");
		exit(2);
	}

	sentence_split(sent, opts);
	int num_linkages = sentence_parse(sent, opts);
	if (num_linkages <= 0) {
		fprintf (stderr, "Fatal error: Unable to parse sentence\n");
		exit(3);
	}

	if (2 < num_linkages) num_linkages = 2;
	for (int li = 0; li<num_linkages; li++)
	{
		Linkage linkage = linkage_create(li, sent, opts);
		linkage_delete(linkage);
	}
	sentence_delete(sent);

	dictionary_delete(dict);
	parse_options_delete(opts);
}

static void parse_sents(int thread_id, int niter)
{
	const char *sents[] = {
		"Frank felt vindicated when his long time friend Bill revealed that he was the winner of the competition.",
		"Logorrhea, or excessive and often incoherent talkativeness or wordiness, is a social disease.",
		"It was covered with bites.",
		"I have no idea what that is.",
		"His shout had been involuntary, something anybody might have done.",
		"Trump, Ryan and McConnell are using the budget process to pay for the GOP???s $1.5 trillion tax scam.",
		"We ate popcorn and watched movies on TV for three days.",
		"Sweat stood on his brow, fury was bright in his one good eye.",
		"One of the things you do when you stop your bicycle is apply the brake.",
		"The line extends 10 miles offshore."
// "?????? ?????????? ?????????????? ???????????? ?????????????????????? ???????????? ??????????????.",
// "?????????? ???????????????? ???????? ?????????? ?????????? ?????????? ??????????????????????."
	};

	int nsents = sizeof(sents) / sizeof(const char *);

	for (int j=0; j<niter; j += nsents)
	{
		for (int i=0; i < nsents; ++i)
		{
			parse_one_sent(sents[i]);
		}
	}
}

int main(int argc, char* argv[])
{
	setlocale(LC_ALL, "en_US.UTF-8");
	dictionary_set_data_dir(DICTIONARY_DIR "/data");

	int n_threads = 10;
	int niter = 30;

	printf("Creating %d threads, each parsing %d sentences\n",
		 n_threads, niter);
	std::vector<std::thread> thread_pool;
	for (int i=0; i < n_threads; i++) {
		thread_pool.push_back(std::thread(parse_sents, i, niter));
	}

	// Wait for all threads to complete
	for (std::thread& t : thread_pool) t.join();
	printf("Done with multi-threaded parsing\n");

	return 0;
}
