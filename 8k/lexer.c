#include "lexer.h"
#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
is_at_end (Lexer *self)
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
  token.line = self->line;
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
    return -3;
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
  return add_token (self, tok);
}
int
ident (Lexer *self, char current)
{
  Token tok;
  size_t len, start = self->current - 1;
  char *ident;

  if (isdigit (peek (self, 0)))
    self->current++;
  len = self->current - start;
  ident = malloc (len + 1);

  if (!ident)
    return -3;
  ident[0] = current;
  ident[1] = len > 1 ? self->input[self->current - 1] : 0;
  if (len > 1)
    ident[2] = 0;

  tok.type = Ident;
  tok.value.s = ident;
  add_token (self, tok);

  return 0;
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
    case '+':
    case '-':
    case '/':
    case '*':
    case '^':
    case '=':
      {
        Token tok;
        char *lit = malloc (2);
        if (!lit)
          return -3;
        lit[0] = current;
        lit[1] = 0;
        tok.type = Operator;
        tok.value.s = lit;
        add_token (self, tok);
      }
      break;
    case '<':
    case '>':
      {
        Token tok;
        char *lit;
        tok.type = Operator;
        if (peek (self, 0) == '=' || (current == '<' && peek (self, 0) == '>'))
          {
            lit = malloc (3);
            if (!lit)
              return -3;
            lit[0] = current;
            lit[1] = advance (self);
            lit[2] = 0;
          }
        else
        {
          lit = malloc (2);
          if (!lit)
            return -3;
          lit[0] = current;
          lit[1] = 0;
        }
        tok.value.s = lit;
        add_token (self, tok);
      }
      break;
    case '(':
      {
        Token tok;
        tok.type = OpenParen;
        add_token (self, tok);
      }
      break;
    case ')':
      {
        Token tok;
        tok.type = CloseParen;
        add_token (self, tok);
      }
      break;
    default:
      if (isalpha (current))
        return ident (self, current);
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
