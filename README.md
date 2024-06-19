# Simple Lambda function that calls tmflib::gen_code() to generate a cryptographic identifier from the given inputs.

Function is stateless and simply returns the results of the gen_code call which includes the formatted code and the full Base32 encoded hash.
