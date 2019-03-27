# menu-lang / mlang

#### Description
Menu-lang or mlang is a simple language that compiles into C. 
It is designed for small and fast scripts that can be written in a short amount of time. 
Functions can be written in C and imported into a menu-lang script. 
```diff
- Note: Some features described in this document may not be implemented yet.
```

#### Design
There are 4 basic datatypes: results, strings, lists, and function pointers. There are a limited amount of data types to keep the language simpler, easier and more straightforward to use.

#### Results
Results or res are the basic return value from a functions. If the function succeded it will return a 0, otherwise it will return a non zero result. results can be stored in a variable or immediately checked in an on block ex:
```
*storing the result of the in function in a variable*
res result_var in input_var
on ok result_var >
  out "printed successfully"
end
```
``` diff
- Note: not implemented yet

*immediately checking the result of the in function*
res in input_var
on ok >
  out "printed successfully"
end
```

#### Strings
Strings are the most used datatype. you can declare a string with the dec command and assign it with the set function:
```
*declare 4 strings*
dec a b c d

*set the string to values*
set a "hello"
set b "world"

*print the strings*
out a
out b
```

All strings are passed by value to functions. Functions may modify or copy the data depending on the implementation, for example: in will set the string sent in to the users input, list_add will copy the string before adding it to a list

#### Lists

#### Function pointers
