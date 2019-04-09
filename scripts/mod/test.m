dec a  b c
dec  var
out  "hello there general kenobi\n"
set var "var"
out_line var
out_line "Enter a word"
res  result in a
on result >
    out "success\n"
	out "your word is: "
	out_line a
	out_line "\nEnter another word"
    in var
end
on result >; out_line "testing results"; end
out "testing "; out_line "semicolons\n"
out_line "\nYour word called from a function pointer is: "
def res pnt str -> out_line
pnt var
