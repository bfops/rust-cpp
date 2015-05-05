all:
	g++ -c mycpplib.cpp -fPIC
	rm -f libmycpplib.a
	ar rvs libmycpplib.a mycpplib.o

clean:
	rm -f *.o *.a
