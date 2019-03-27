" Vim syntax file
" Language: mlang
" Maintainer: Alex Burroughs
" Latest Revision: 27 March 2019

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword celBlockCmd dec def list on res while  mod

" Cell blocks
syn keyword celDescBlock str ls

" Strings
syn region celString start='"' end='"' contained
syn region celDesc start='"' end='"'

syn region celComment start='*' end='*' contained
syn region celDescComm start='*' end='*'

let b:current_syntax = "cel"

hi def link celComment     Comment
hi def link celBlockCmd    Statement
hi def link celString      Constant
hi def link celDesc        PreProc    
hi def link celDescBlock   Type
