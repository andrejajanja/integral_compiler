# Real Function Approximation Around a Point with Taylor Polynomials Using Custom-Compiled Machine Code

<img src = "https://logodix.com/logo/1784092.jpg" style = "width: 60%; height: 60%; margin-right: 2rem;">

## Overview

### How the whole thing works? - **TL;DR**
1. User provides number of samples and the function string (_eg. f(x)=`sin(x)*exp(x)+ln(x)*tan(x)`_),
2. Provided function is parsed, optimized and compiled to LLVM IR, 
3. LLVM Backend then compiles generated IR to native machine code and emmits it to object file savet to a buffer in operating memory (refered to as *object buffer*),
4. This object buffer is dinamically linked against the definitons of used mathematical functions (using host **OS's standard lib** or my custom implementations),
5. Object buffer has the `f(x)` symbol, pointer to the it's location is returned as a result of linking process,
6. This generated function is then used in unsafe Rust code to numerically approximate the integral using trapezoidal formula.

**White paper explaining the entire project in great detail is in the file taylor_compiler.pdf**

## License

This project is licensed under the CC BY-NC 4.0 License. Non-commercial use is allowed with attribution.
For commercial use, please contact me, Andreja Janković via e-mail: andrejajanja@gmail.com
