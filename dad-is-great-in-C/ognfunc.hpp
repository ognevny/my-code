#pragma GCC diagnostic ignored "-Wmacro-redefined"

#ifdef __GNUC__
#define _GLIBCXX_USE_CXX11_ABI 0
#endif

#include <string>

unsigned long long numsum(unsigned long long n);
unsigned long long reverse(unsigned long long n);
unsigned long long numsys(unsigned long long n, unsigned char k);
unsigned short numcount(unsigned long long n);
std::string numsys16(unsigned long long n, unsigned char k);
long double binpow(long double b, unsigned short e);
