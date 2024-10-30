# Numerical Integral Approximation Using Compiler Based Approach

<div style = "display: flex; flex-direction: row;">
    <img src = "https://logodix.com/logo/1784092.jpg" style = "width: 100%; height: 100%; margin-right: 2rem;">
    <img src = "https://th.bing.com/th/id/OIP.tNft0ETIabqB2AtBx7ZQJwHaE8?rs=1&pid=ImgDetMain" style = "width: 45.65%;">
</div>

## Overview

### How this project came to be?
I wanted to make a program that would numerically evaluate<br>
![Integral](https://latex.codecogs.com/png.latex?\color{White}\int_{a}^{b}%20f(x)%20dx%20\quad%20x,%20a,%20b%20\in%20\mathbb{R}\quad%20a%20%3C%20b)
<br>for ```f(x)``` being **any** function comprised of standard, real, math functions.

Additionaly, I didn't want to rely on any external tools, therefore external compiler (*eg. gcc*) or *JIT* based approach was out of the question. In the end, I concluded I need to write my own **Just In Time Compiler**.

### How the whole thing works? - **TL;DR**
1. User provides number of samples and the function string (_eg. f(x)=`sin(x)*exp(x)+ln(x)*tan(x)`_),
2. Provided function is parsed, optimized and compiled to LLVM IR, 
3. LLVM Backend then compiles generated IR to native machine code and emmits it to object file savet to a buffer in operating memory (refered to as *object buffer*),
4. This object buffer is dinamically linked against the definitons of used mathematical functions (using host **OS's standard lib** or my custom implementations),
5. Object buffer has the `f(x)` symbol, pointer to the it's location is returned as a result of linking process,
6. This generated function is then used in unsafe Rust code to numerically approximate the integral using trapezoidal formula.

## Software architecture

Will subsequently add this section

## Custom optimizations used

Will subsequently add this section

## Instalation and supported targets
Will subsequently add this section

## License

This project is licensed under the CC BY-NC 4.0 License. Non-commercial use is allowed with attribution.
For commercial use, please contact me, Andreja Janković via e-mail: andrejajanja@gmail.com
