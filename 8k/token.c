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
    }
  puts ("");
}