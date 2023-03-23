cmake_minimum_required(VERSION 3.17)

project(openssl_if LANGUAGES C CXX)



set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_VERBOSE_MAKEFILE ON)

set(CMAKE_INCLUDE_CURRENT_DIR ON)## Always include the current directory

find_package(OpenSSL REQUIRED)
if (OPENSSL_FOUND)
    message(STATUS, "OPENSSL Version -> ${OPENSSL_VERSION}")
    message(STATUS, "OPENSSL INC DiR -> ${OPENSSL_INCLUDE_DIR}")
    message(STATUS, "OPENSSL LIBRARY -> ${OPENSSL_LIBRARIES}")
endif()

set(CMAKE_VERBOSE_MAKEFILE ON)

add_library(openssl_lib SHARED ${CMAKE_CURRENT_SOURCE_DIR}/conversions.cc 
            ${CMAKE_CURRENT_SOURCE_DIR}/openssl_if.cc 
            ${CMAKE_CURRENT_SOURCE_DIR}/conversions.h 
            ${CMAKE_CURRENT_SOURCE_DIR}/openssl_if.h)
target_link_libraries(openssl_lib OpenSSL::Crypto)

add_executable(TestIFLib ${CMAKE_CURRENT_SOURCE_DIR}/test_openssl_if.cc)
target_include_directories(TestIFLib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(TestIFLib PRIVATE openssl_lib)

install(TARGETS openssl_lib
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
