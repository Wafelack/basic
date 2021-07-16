#include "lexer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <errno.h>

Lexer 
lexer_new (const char *input, const char *file)
{
  Lexer lexer;
  lexer.input = input;
  lexer.file = file;
  lexer.start = 0;
  lexer.current = 0;
  lexer.line = 1;
  lexer.count = 0;
  lexer.output = NULL;
  return lexer;
}
char
is_at_end(Lexer *self)
{
  return self->current >= strlen (self->input);
}

char
peek (Lexer *self, int count)
{
  if (self->current + count < strlen (self->input))
    return self->input[self->current + count];
  else
    return -1;
}
char
advance (Lexer *self)
{
  if (!is_at_end (self))
    return self->input[self->current++];
  else
    return -1;
}
int
add_token (Lexer *self, Token token)
{
  self->output = realloc (self->output, (self->count + 1) * sizeof (Token));
  if (!self->output)
    return -3;
  self->output[self->count++] = token;
  return 0;
}
int
number (Lexer *self)
{
  char current, *raw;
  double val;
  uint32_t i;
  Token tok;
  while (current = peek (self, 0), current != -1 && isdigit (current))
    self->current++;
  if (peek (self, 0) == '.')
    self->current++;
  while (current = peek (self, 0), current != -1 && isdigit (current))
    self->current++;

  raw = malloc (self->current - self->start + 1);
  if (!raw)
    return -1;
  for (i = self->start; i < self->current; i++)
    raw[i - self->start] = self->input[i];
  errno = 0;
  val = atof (raw);
  if (!val && errno)
    {
      free (raw);
      return -2;
    }
  free (raw);
  tok.type = Number;
  tok.value.number = val;
  tok.line = self->line;
  return add_token (self, tok);
}
int
lex_token (Lexer *self)
{
  char current;
  if (current = advance (self), current == -1)
    return -1;

  if (isdigit (current))
    return number (self);
  
  switch (current)
  {
    case '\n':
      self->line++;
      break;
    case '\r': 
    case ' ': 
    case '\t': 
      break;
    default:
      printf ("%c\n", current);
      break;
  }
  return 0;
}
int
lex (Lexer *self)
{
  while (!is_at_end (self))
  {
    int ret = lex_token (self);
    if (ret)
      return ret;
    self->start = self->current;
  }
  return 0;
}
void
lexer_print_err (Lexer lexer, int code)
{
  fprintf (stderr, "%s:%u: ", lexer.file, lexer.line);
  switch (code)
  {
    case -1:
      fprintf (stderr, "unexpected EOF.");
      break;
    case -2:
      fprintf (stderr, "invalid number.");
      break;
    case -3:
      fprintf (stderr, "allocation failed.");
      break;
    default:
      fprintf (stderr, "unknown error code: %d.", code);
      break;
  }
  fprintf (stderr, "\n");
}
