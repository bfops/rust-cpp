#ifndef cpp_H
#define cpp_H

#include <cstdio>

void foo(int to_print);

template<typename T>
T bar() {
  T ret;
  printf("[C++] bar returning %d\n", ret);
  return ret;
}

#endif
