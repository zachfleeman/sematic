use strum_macros::{Display, EnumString};

// Reference: https://sites.google.com/site/partofspeechhelp/
// LanguageTool tagset: https://github.com/languagetool-org/languagetool/blob/master/languagetool-language-modules/en/src/main/resources/org/languagetool/resource/en/tagset.txt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum TreebankPOS {
  CC,     // Coordinating conjunction: and, or, either, if, as, since, once, neither, less
  CD,     // Cardinal number: one, two, twenty-four
  DT,     // Determiner: a, an, all, many, much, any, some, this
  EX,     // Existential there: there (no other words)
  FW,     // Foreign word: infinitum, ipso
  IN,     // Preposition/subordinate conjunction: except, inside, across, on, through, beyond, with, without
  JJ,     // Adjective: beautiful, large, inspectable
  JJR,    // Adjective, comparative: larger, quicker
  JJS,    // Adjective, superlative: largest, quickest
  LS,     // List item marker: not used by LanguageTool
  MD,     // Modal: should, can, need, must, will, would
  NN,     // Noun, singular count noun: bicycle, earthquake, zipper
  NNS,    // Noun, plural: bicycles, earthquakes, zippers
  #[strum(serialize = "NN:U")]
  NNU,    // Nouns that are always uncountable		#new tag - deviation from Penn, examples: admiration, Afrikaans
  #[strum(serialize = "NN:UN")]
  NNUN,   // Nouns that might be used in the plural form and with an indefinite article, depending on their meaning	#new tag - deviation from Penn, examples: establishment, wax, afternoon
  NNP,    // Proper noun, singular: Denver, DORAN, Alexandra
  NNPS,   // Proper noun, plural: Buddhists, Englishmen
  ORD,    // Ordinal number: first, second, twenty-third, hundredth #New tag (experimental) since LT 4.9. Specified in disambiguation.xml. Examples: first, second, third, twenty-fourth, seventy-sixth
  PCT,    // Punctuation mark: (`.,;:…!?`) #new tag - deviation from Penn
  PDT,    // Predeterminer: all, sure, such, this, many, half, both, quite
  POS,    // Possessive ending: s (as in: Peter's)
  PRP,    // Personal pronoun: everyone, I, he, it, myself
  #[strum(serialize = "PRP$")]
  PRP2,   // Possessive pronoun: its, our, their, mine, my, her, his, your
  RB,     // Adverb and negation: easily, sunnily, suddenly, specifically, not
  RBR,    // Adverb, comparative: better, faster, quicker
  RBS,    // Adverb, superlative: best, fastest, quickest
  #[strum(serialize = "RB_SENT")]
  RBSENT, // Adverbial phrase including a comma that starts a sentence. #New tag (experimental) since LT 4.8. Specified in disambiguation.xml. Examples: However, Whenever possible, First of all, On the other hand,
  RP,     // Particle: in, into, at, off, over, by, for, under
  SYM,    // Symbol: not used by LanguageTool
  TO,     // to: to (no other words)
  UH,     // Interjection: aargh, ahem, attention, congrats, help
  VB,     // Verb, base form: eat, jump, believe, be, have
  VBD,    // Verb, past tense: ate, jumped, believed
  VBG,    // Verb, gerund/present participle: eating, jumping, believing
  VBN,    // Verb, past participle: eaten, jumped, believed
  VBP,    // Verb, non-3rd ps. sing. present: eat, jump, believe, am (as in 'I am'), are
  VBZ,    // Verb, 3rd ps. sing. present: eats, jumps, believes, is, has
  WDT,    // wh-determiner: that, whatever, what, whichever, which (no other words)
  WP,     // wh-pronoun: that, whatever, what, whatsoever, whomsoever, whosoever, who, whom, whoever, whomever, which (no other words)
  #[strum(serialize = "WP$")]
  WP2,    // Possessive wh-pronoun: whose (no other words)
  WRB,    // wh-adverb: however, how, wherever, where, when, why
}

/*
Omitted for now:
``    Left open double quote
,     Comma
''    Right close double quote
.     Sentence-final punctuation (in LanguageTool, use SENT_END instead)
:     Colon, semi-colon
$     Dollar sign
#     Pound sign
 */
