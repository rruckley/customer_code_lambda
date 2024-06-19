# Simple Lambda function for tmflib::gen_code()

Function is stateless and simply returns the results of the gen_code call which includes the formatted code and the full Base32 encoded hash.

Calls [tmflib::gen_code()](https://docs.rs/tmflib/0.1.17/tmflib/fn.gen_code.html) with inputs from the payload and returns output of same.
