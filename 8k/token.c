#include "token.h"
#include <stdio.h>

void
token_print (Token token)
{
  printf ("[%u] ", token.line);
  switch (token.type)
    {
    case Number:
      printf ("Number: %f", token.value.number);
      break;
    case Ident:
      printf ("Ident: %s", token.value.s);
      break;
    case Operator:
      printf ("Operator: %c", token.value.c);
      break;
    case OpenParen:
      printf ("(");
      break;
    case CloseParen:
      printf (")");
      break;
    }
  puts ("");
}
