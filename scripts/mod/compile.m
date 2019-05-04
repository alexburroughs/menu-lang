dec command
expect "1" "Usage: Enter a program name"
collect filename

*build the command*
set command "menu-lang "
strcat command filename
strcat command ".m "
strcat command filename
strcat command ".c > /dev/null"

res result sys command

on result >
	set command "gcc "
	strcat command filename
	strcat command ".c "
	strcat command "std.c arraylist.c"
	strcat command " -std=c99 "
	strcat command "-o "
	strcat command filename
	sys command
end
