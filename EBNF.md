var =  string [digits] {string [digits]}
exp = none | true | false | digits | literal | (exp | var) | exp binop exp | unop exp

digits = "1" .. "9" {"1" .. "9"}
string = ("a" ..  "z" | "A" .. "Z") {("a" ..  "z" | "A" .. "Z")}
literal = ' " ' {string | digits}' " '

binop = ">=" | "<=" | "\==" | "+" | "-" | "\*" | "/" | ">" | "<" | and | or
unop = "-" | not 

return = return (exp | var)
statement = var "=" exp | break | continue 
|
while exp "{" block "}" 
|
if exp "{" block "}" {elif exp "{" block "}"} [else "{" block "}"] 
|
for var in exp "{" block "}" 

block = {statement} [return]
