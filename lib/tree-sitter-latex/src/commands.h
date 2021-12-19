#include <stdlib.h>

char* citation_commands[] = {
    "cite",        "cite*",       "Cite",      "nocite",       "citet",
    "citep",       "citet*",      "citep*",    "citeauthor",   "citeauthor*",
    "Citeauthor",  "Citeauthor*", "citetitle", "citetitle*",   "citeyear",
    "citeyear*",   "citedate",    "citedate*", "citeurl",      "fullcite",
    "citeyearpar", "citealt",     "citealp",   "citetext",     "parencite",
    "parencite*",  "Parencite",   "footcite",  "footfullcite", "footcitetext",
    "textcite",    "Textcite",    "smartcite", "Smartcite",    "supercite",
    "autocite",    "Autocite",    "autocite*", "Autocite*",    "volcite",
    "Volcite",     "pvolcite",    "Pvolcite",  "fvolcite",     "ftvolcite",
    "svolcite",    "Svolcite",    "tvolcite",  "Tvolcite",     "avolcite",
    "Avolcite",    "notecite",    "notecite",  "pnotecite",    "Pnotecite",
    "fnotecite",   NULL};

char* label_reference_commands[] = {
    "ref",         "eqref",     "vref",          "Vref",      "autoref",
    "pageref",     "cref",      "Cref",          "cref*",     "Cref*",
    "namecref",    "nameCref",  "lcnamecref",    "namecrefs", "nameCrefs",
    "lcnamecrefs", "labelcref", "labelcpageref", NULL};

char* label_reference_range_commands[] = {
    "crefrange",  "crefrange",  "Crefrange",  "Crefrange", "crefrange*",
    "crefrange*", "Crefrange*", "Crefrange*", NULL};

char* new_command_definition_commands[] = {
    "newcommand",          "newcommand*",          "renewcommand",
    "renewcommand*",       "DeclareRobustCommand", "DeclareRobustCommand*",
    "DeclareMathOperator", "DeclareMathOperator*", NULL};

char* glossary_entry_reference_commands[] = {"gls",
                                             "Gls",
                                             "GLS",
                                             "glspl",
                                             "Glspl",
                                             "GLSpl",
                                             "glsdisp",
                                             "glslink",
                                             "glstext",
                                             "Glstext",
                                             "GLStext",
                                             "glsfirst",
                                             "Glsfirst",
                                             "GLSfirst",
                                             "glsplural",
                                             "Glsplural",
                                             "GLSplural",
                                             "glsfirstplural",
                                             "Glsfirstplural",
                                             "GLSfirstplural",
                                             "glsname",
                                             "Glsname",
                                             "GLSname",
                                             "glssymbol",
                                             "Glssymbol",
                                             "glsdesc",
                                             "Glsdesc",
                                             "GLSdesc",
                                             "glsuseri",
                                             "Glsuseri",
                                             "GLSuseri",
                                             "glsuserii",
                                             "Glsuserii",
                                             "GLSuserii",
                                             "glsuseriii",
                                             "Glsuseriii",
                                             "GLSuseriii",
                                             "glsuseriv",
                                             "Glsuseriv",
                                             "GLSuseriv",
                                             "glsuserv",
                                             "Glsuserv",
                                             "GLSuserv",
                                             "glsuservi",
                                             "Glsuservi",
                                             "GLSuservi",
                                             NULL};

char* acronym_reference_commands[] = {"acrshort",
                                      "Acrshort",
                                      "ACRshort",
                                      "acrshortpl",
                                      "Acrshortpl",
                                      "ACRshortpl",
                                      "acrlong",
                                      "Acrlong",
                                      "ACRlong",
                                      "acrlongpl",
                                      "Acrlongpl",
                                      "ACRlongpl",
                                      "acrfull",
                                      "Acrfull",
                                      "ACRfull",
                                      "acrfullpl",
                                      "Acrfullpl",
                                      "ACRfullpl",
                                      "acs",
                                      "Acs",
                                      "acsp",
                                      "Acsp",
                                      "acl",
                                      "Acl",
                                      "aclp",
                                      "Aclp",
                                      "acf",
                                      "Acf",
                                      "acfp",
                                      "Acfp",
                                      "ac",
                                      "Ac",
                                      "acp",
                                      "glsentrylong",
                                      "Glsentrylong",
                                      "glsentrylongpl",
                                      "Glsentrylongpl",
                                      "glsentryshort",
                                      "Glsentryshort",
                                      "glsentryshortpl",
                                      "Glsentryshortpl",
                                      "glsentryfullpl",
                                      "Glsentryfullpl",
                                      NULL};

char* theorem_definition_commands[] = {"newtheorem", "declaretheorem", NULL};

char* color_reference_commands[] = {"color", "colorbox", "textcolor",
                                    "pagecolor", NULL};

char* tikz_library_import_commands[] = {"usepgflibrary", "usetikzlibrary",
                                        NULL};

char* package_include_commands[] = {"usepackage", "RequirePackage", NULL};

char* latex_include_commands[] = {"include", "subfileinclude", "input",
                                  "subfile", NULL};

char* verbatim_include_commmands[] = {"verbatiminput", "VerbatimInput", NULL};

char* import_commands[] = {
    "import",      "subimport",      "inputfrom", "subimportfrom",
    "includefrom", "subincludefrom", NULL};

char* part_commands[] = {"part", "part*", NULL};

char* chapter_commands[] = {"chapter", "chapter*", NULL};

char* section_commands[] = {"section", "section*", NULL};

char* subsection_commands[] = {"subsection", "subsection*", NULL};

char* subsubsection_commands[] = {"subsubsection", "subsubsection*", NULL};

char* paragraph_commands[] = {"paragraph", "paragraph*", NULL};

char* subparagraph_commands[] = {"subparagraph", "subparagraph*", NULL};

char* enum_item_commands[] = {"item", NULL};
