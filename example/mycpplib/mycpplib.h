#ifndef mycpplib_H
#define mycpplib_H

#include <cstdio>

template <typename T>
struct Foo {
  T x_;

  Foo() {}

  Foo(T x) {
    x_ = x;
  }

  virtual ~Foo() {
    printf("[C++] ~Foo called\n");
  }
};

void foo(int to_print);

template <typename T>
T bar() {
  T ret;
  printf("[C++] bar returning %d\n", ret);
  return ret;
}

#endif
