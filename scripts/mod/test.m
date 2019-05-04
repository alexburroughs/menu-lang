*declare variables*
dec  a b c
dec  var

out  "Welcome to the demo\n"
set var "var"
out "setting a variable test: "
out_line var

*prompt the user and get a result* 
out_line "Enter a word"
res result in a

*if the in command was successful*
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

*make a function pointer to out_line*
def res pnt str -> out_line
pnt var

out_line "Enter a system command"
in var

*run the users system command*
res sys_out sys var

*Keep running the users commands until it reaches an error*
while sys_out >
	out_line "Enter another command:"
	in var
	res sys_out sys var
end

out_line "exiting..."

