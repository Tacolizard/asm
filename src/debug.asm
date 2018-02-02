mov 300 $0
mov 400 $0
add 400 $1
mov 5 $0xFFFFFFFF
add 300 $1
cmp 300 $70
jne 9
mov 300 $0
mov 5 $0x79283e
add 300 $1
cmp 300 $70
jne 14
mov 300 $0
mov 5 $0xFFFFFFFF
add 300 $1
cmp 300 $70
jne 19
jmp 6
eof
