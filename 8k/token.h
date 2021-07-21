#ifndef _8K_TOKEN_H
#define _8K_TOKEN_H

#include <stdint.h>

typedef enum ttype
{
  Number,
  Ident,
  Operator,
  OpenParen,
  CloseParen,
  Let,
  Print
} TType;

typedef struct token
{
  TType type;
  uint32_t line;
  union
  {
    double number;
    char *s;
  } value;
} Token;

void token_print (Token);

#endif
