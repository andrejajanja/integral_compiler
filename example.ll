target triple = "x86_64-pc-linux-gnu"
;target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)

define i32 @main() {
    %num1 = alloca i32
    %num2 = alloca i32

    store i32 10, i32* %num1
    store i32 20, i32* %num2

    %val1 = load i32, i32* %num1
    %val2 = load i32, i32* %num2
    %sum = add i32 %val1, %val2

    ret i32 %sum
}