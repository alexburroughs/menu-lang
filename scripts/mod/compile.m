dec filename command
out_line "Enter a filename"
set filename "test"

*build the command*
set command "menu-lang "
strcat command filename
strcat command ".m "
strcat command filename
strcat command ".c"

res result sys command

on result >
	set command "gcc *.c -std=c99 "
	strcat command "-o "
	strcat command filename
	sys command
end
