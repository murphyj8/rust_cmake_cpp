#ifndef __OPENSSL_IF_H__
#define __OPENSSL_IF_H__


#include <memory>

std::unique_ptr<unsigned char []> HashIt(const std::unique_ptr<unsigned char[]>& msg, const int& len, int& digestlen);

extern "C"{
    bool Hash256_To_StdOut(const char*);
    const char * Hash256(const char *); 
}
#endif