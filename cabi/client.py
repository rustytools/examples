# -*- coding: utf-8 -*-

import ctypes

if "__main__" == __name__:
    lib = ctypes.CDLL("./target/debug/libmy.so")
    lib.hello.restype = ctypes.c_char_p
    out = ctypes.c_char_p()
    out_len = ctypes.c_int();
    name = "mighty python"
    err = lib.hello(ctypes.c_char_p(name), ctypes.c_int(len(name)),
            ctypes.byref(out), ctypes.byref(out_len))
    if(err is not None):
        print("error: " + err.value)
        lib.hello_free(err)
    print(out.value)
    lib.hello_free(out)
