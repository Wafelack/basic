#include "utils.h"
#include <stdio.h>

void
print_err (const char *file, uint32_t line, int code)
{
  fprintf (stderr, "%s:%u: ", file, line);
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
