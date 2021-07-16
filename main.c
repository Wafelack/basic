#include "8k/lexer.h"
#include <stdlib.h>

int
main (void)
{
  Lexer lexer = lexer_new ("42 3.14159265358926535897932", "test");
  int err;
  uint32_t i;
  if (err = lex (&lexer), err)
    {
      lexer_print_err (lexer, err);
      return EXIT_FAILURE;
    }
  for (i = 0; i < lexer.count; i++)
    token_print (lexer.output[i]);
  free (lexer.output);
  return EXIT_SUCCESS;
}
