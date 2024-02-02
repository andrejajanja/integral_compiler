llc -O0 -filetype=obj IR_code.ll -o ir_out.o
ld.lld ir_out.o -o tst -L/lib/x86_64-linux-gnu -lm
./tst