#include <openssl/bn.h>
#include <openssl/evp.h>
#include <openssl/ec.h>
#include <openssl/objects.h>
#include <openssl/sha.h>

#include <openssl_if.h>
#include <conversions.h>
#include <iostream>
#include <string>

std::unique_ptr<unsigned char []> HashIt(const std::unique_ptr<unsigned char[]>& msg, const int& len, int& digestlen){
    SHA256_CTX ctx;
    std::unique_ptr<unsigned char []> digest (new unsigned char[SHA256_DIGEST_LENGTH]);
    SHA256_Init(&ctx);
    SHA256_Update(&ctx, msg.get(), len);
    SHA256_Final(digest.get(), &ctx);
    OPENSSL_cleanse(&ctx, sizeof(ctx));
    digestlen = SHA256_DIGEST_LENGTH;
    return digest;
}

bool Hash256_To_StdOut(const char* msg){
    const std::string& message(msg); 
    std::unique_ptr<unsigned char[]> message_bin(new unsigned char[message.length() + 1]);
    std::fill_n(message_bin.get(),message.length()+1, 0x00);
    int index(0);
    for(std::string::const_iterator iter = message.begin(); iter != message.end(); ++ iter){
        message_bin[index++] = *iter; 
    }

    int digestlen(0); 
    std::unique_ptr<unsigned char[]> digest = HashIt(message_bin, index, digestlen);
    std::string hexDigest = binTohexStr(digest,digestlen); 
    std::cout << hexDigest << std::endl;
    return true;
}

const char * Hash256(const char * msg){
    const std::string& message(msg); 
    std::unique_ptr<unsigned char[]> message_bin(new unsigned char[message.length() + 1]);
    std::fill_n(message_bin.get(),message.length()+1, 0x00);
    int index(0);
    for(std::string::const_iterator iter = message.begin(); iter != message.end(); ++ iter){
        message_bin[index++] = *iter; 
    }

    int digestlen(0); 
    std::unique_ptr<unsigned char[]> digest = HashIt(message_bin, index, digestlen);
    std::string hexDigest = binTohexStr(digest,digestlen); 
    char * ret = new char[hexDigest.length()]; 
    strcpy(ret, hexDigest.c_str());
    return ret;
}
