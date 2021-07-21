#include "8k/lexer.h"
#include "utils.h"
#include <stdlib.h>

int
main (void)
{
  Lexer lexer = lexer_new ("LET x = (K9 + 55) * 3.1415 ^ 14\n3 = 1 + 1  78 <= 99 52 <> 3.14\nPRINT x", "test");
  int err;
  uint32_t i;
  if (err = lex (&lexer), err)
    {
      print_err (lexer.file, lexer.line, err);
      return EXIT_FAILURE;
    }
  for (i = 0; i < lexer.count; i++)
    token_print (lexer.output[i]);
  destroy_lexer (lexer);
  return EXIT_SUCCESS;
}
