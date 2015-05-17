#ifndef cpp_H
#define cpp_H

#include <cstdio>

struct Foo {
  int x_;
};

template <typename T>
struct Bar {
  Foo foo_;
  T x_;
};

void foo(int to_print);

template <typename T>
T bar() {
  T ret;
  printf("[C++] bar returning %d\n", ret);
  return ret;
}

#endif
