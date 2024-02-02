llc -O0 -filetype=obj IR_code.ll -o ir_out.o
gcc ir_out.o -o executable -L/lib/x86_64-linux-gnu -lm -no-pie