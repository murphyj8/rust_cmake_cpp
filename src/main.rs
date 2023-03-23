
use libc::c_char;
use std::ffi::{CString,CStr};

#[link(name = "openssl_lib", kind = "dylib")]
extern "C" {

    fn Hash256_To_StdOut(message: *const c_char) -> bool;
    fn Hash256(message: *const c_char) -> *const c_char; 
}

fn main() {
    println!("Hello, world!");
    unsafe{
        let c_string = CString::new("The time has now come for us to ensure that we not only continue our amazing activities in R&D, but also increase our investment in this area and grow our patent output. This can only be achieved by monetizing our business and actively driving forward the company's commercial operation turning the Group into a consultancy, advisory and commercial powerhouse that cannot be ignored. We will provide best in class technological products and solutions to enterprises and governments across the globe. This is a challenge I relish. \
        Since my appointment, I have set out to review the existing business and organisational structure of the Group to ensure we can build a strong and resilient nChain which, as the DNA of Blockchain, is able to take the rightful commercial lead in a fast-changing world involving Web 3, smart contracts and other smart applications including nano transactions and payment solutions. Moreover, the unique scalability of the Bitcoin (BSV) Blockchain protocol gives us the potential to considerably propel our growth over the coming months and years.").expect("CString::new failed");
        let ptr = c_string.into_raw();

        let ret = Hash256_To_StdOut(ptr);

        // retake the pointer from memory
        let _ = CString::from_raw(ptr);
        println!("{}", ret);

        let c_string = CString::new("The time has now come for us to ensure that we not only continue our amazing activities in R&D, but also increase our investment in this area and grow our patent output. This can only be achieved by monetizing our business and actively driving forward the company's commercial operation turning the Group into a consultancy, advisory and commercial powerhouse that cannot be ignored. We will provide best in class technological products and solutions to enterprises and governments across the globe. This is a challenge I relish. \
        Since my appointment, I have set out to review the existing business and organisational structure of the Group to ensure we can build a strong and resilient nChain which, as the DNA of Blockchain, is able to take the rightful commercial lead in a fast-changing world involving Web 3, smart contracts and other smart applications including nano transactions and payment solutions. Moreover, the unique scalability of the Bitcoin (BSV) Blockchain protocol gives us the potential to considerably propel our growth over the coming months and years.").expect("CString::new failed");
        let ptr = c_string.into_raw();
        let char_ptr = Hash256(ptr);
        let c_str: &CStr = CStr::from_ptr(char_ptr);

        let _ =  CString::from_raw(ptr); 
        println!("{:?}", c_str); 
    }

}
