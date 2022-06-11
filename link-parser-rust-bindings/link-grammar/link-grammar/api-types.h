/*************************************************************************/
/* Copyright (c) 2004                                                    */
/* Daniel Sleator, David Temperley, and John Lafferty                    */
/* All rights reserved                                                   */
/*                                                                       */
/* Use of the link grammar parsing system is subject to the terms of the */
/* license set forth in the LICENSE file included with this software.    */
/* This license allows free redistribution and use in source and binary  */
/* forms, with or without modification, subject to certain conditions.   */
/*                                                                       */
/*************************************************************************/

/* This file is somewhat misnamed, as everything here is private --
 * these are the internal-use-only API objects interfacing between
 * different subsystems.  They are all subject to change without notice.
 * No external code should attempt to access this stuff.
 */

#ifndef _API_TYPES_H_
#define _API_TYPES_H_

/* Widely used typedefs */
typedef struct Exp_struct Exp;
typedef struct Connector_struct Connector;
typedef struct Linkage_info_struct Linkage_info;
typedef struct Postprocessor_s Postprocessor;
typedef struct Resources_s * Resources;

/* Some of the more obscure typedefs */
typedef struct Disjunct_struct Disjunct;
typedef struct Link_s Link;
typedef struct String_set_s String_set;
typedef struct String_id_s String_id;
typedef struct Word_struct Word;
typedef struct Gword_struct Gword;
typedef struct gword_set gword_set;
typedef struct tracon_sharing_s Tracon_sharing;
typedef struct Dialect_s Dialect;
typedef struct Word_file_struct Word_file;
typedef struct Wordgraph_pathpos_s Wordgraph_pathpos;

/* Post-processing structures */
typedef struct pp_knowledge_s pp_knowledge;
typedef struct pp_linkset_s pp_linkset;
typedef struct PP_domains_s PP_domains;

#endif
