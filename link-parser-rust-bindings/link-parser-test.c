#include <locale.h>
#include <stdio.h>
// #include "link-includes.h"
#include "./link-grammar/link-grammar/link-includes.h"

int main()
{
  Dictionary dict;
  Parse_Options opts;
  Sentence sent;
  Linkage linkage;
  char *diagram;
  int i, num_linkages;
  const char *input_string[] = {
      "Grammar is useless because there is nothing to say -- Gertrude Stein.",
      "Computers are useless; they can only give you answers -- Pablo Picasso."};

  setlocale(LC_ALL, "");
  opts = parse_options_create();
  dict = dictionary_create_lang("en");
  if (!dict)
  {
    printf("Fatal error: Unable to open the dictionary\n");
    return 1;
  }

  for (i = 0; i < 2; ++i)
  {
    sent = sentence_create(input_string[i], dict);
    sentence_split(sent, opts);
    num_linkages = sentence_parse(sent, opts);
    if (num_linkages > 0)
    {
      linkage = linkage_create(0, sent, opts);
      printf("%s\n", diagram = linkage_print_diagram(linkage, true, 800));
      linkage_free_diagram(diagram);
      linkage_delete(linkage);
    }
    sentence_delete(sent);
  }

  dictionary_delete(dict);
  parse_options_delete(opts);
  return 0;
}