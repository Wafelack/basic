#ifndef _8K_LEXER_H
#define _8K_LEXER_H

#include "token.h"

typedef struct lexer
{
  const char *input, *file;
  uint32_t count, start, current, line;
  Token *output;
} Lexer;

Lexer lexer_new (const char *, const char *);
void destroy_lexer (Lexer);
int lex (Lexer *);

#endif
