# Pajson 🥧 
*an emoji language from the future*


## Pajson syntax in BNF
---

```
<comment> ::= "🙈" <eol> | "🙈" <whatever> <eol>

<statements> ::= <statements> <statement> | <statement>

<statement> ::= <assignment> | <if-statement> | <print> | <loop> | <break-loop>

<print> ::= "💬" "✋" <expr> "🤚" <eol>

<assignment> ::= <emojis> "👈" <expr> <eol>

<expr> ::= <expr> “➕” <term> | <expr> “➖” <term> | <term>

<term> ::= <term> “❎” <factor> | <term> “➗” <factor> | <factor>

<factor> ::= “✋“ <expr> “🤚” | “➖” <factor> | <integer> | <identifier>

<identifier> ::= <emojis> | <int>

<int> ::= <int> <digit> | <digit>

<if-statement> ::= "👀" <expr> <cmp> <expr> "🫳" <nl> <statements> <nl> "🫴" <eol>

<cmp> ::= "👉👈" | "🐊" | "🐰" | "👎🐊" | "👎🐰" | "👎👈"

<loop> ::= "🔄" "🫳" <nl> <statements> <nl> "🫴" <eol>

<break-loop> ::= "🔚" <eol>

<eol> ::= <nl> | <etx>

<nl> ::= "U+000D"

<etx> ::= "U+2403"

<digit> ::= "🕛" | "🕐" | "🕑" | "🕒" | "🕓" | "🕔" | "🕕" | "🕖" | "🕗" | "🕘"

<emojis> ::= <emojis> <emoji> | <emoji>

<emoji> ::= "😀" |"😃" |"😄" |"😁" |"😆" |"🥹" |"😅" |"😂" |"🤣" |"🥲" |"😊" |"😇" |"🙂" |"🙃" |"😉" |"😌" |"😍" |"🥰" |"😘" |"😗" |"😙" |"😚" |"😋" |"😛" |"😝" |"😜" |"🤪" |"🤨" |"🧐" |"🤓" |"😎" |"🥸" |"🤩" |"🥳" |"😏" |"😒" |"😞" |"😔" |"😟" |"😕" |"🙁" |"😣" |"😖" |"😫" |"😩" |"🥺" |"😢" |"😭" |"😤" |"😠" |"😡" |"🤬" |"🤯" |"😳" |"🥵" |"🥶" |"😱" |"😨" |"😰" |"😥" |"😓" |"🤗" |"🤔" |"🫣" |"🤭" |"🫢" |"🫡" |"🤫" |"🫠" |"🤥" |"😶" |"🫥" |"😐" |"🫤" |"😑" |"😬" |"🙄" |"😯" |"😦" |"😧" |"😮" |"😲" |"🥱" |"😴" |"🤤" |"😪" |"😵" |"🤐" |"🥴" |"🤢" |"🤮" |"🤧" |"😷" |"🤒" |"🤕" |"🤑" |"🤠" |"😈" |"👿" |"👹" |"👺" |"🤡" |"💩" |"👻" |"💀" |"👽" |"👾" |"🤖" |"🎃" |"😺" |"😸" |"😹" |"😻" |"😼" |"😽" |"🙀" |"😿" |"😾"
```
---
## Fibonacci in Pajson
```
🙈 fibonacci
😘👈🕛
😍👈🕐
🤓👈🕛
😡👈🕐🕛

💬✋😘🤚

🔄🫳
    💬✋😍🤚

    🙈 update a_{i}, a_{i+1}
    💀👈😘➕😍
    😘👈😍
    😍👈💀

    🙈 increment counter 🤓 and check if exit condition is met
    🤓👈🤓➕🕐
    👀🤓🐊😡🫳
        🔚
    🫴
🫴
```

---
## Transpiles to C
```
#include <stdio.h>
int main() {
int v0 = 0;
int v1 = 1;
int v2 = 0;
int v3 = 10;
printf("%d\n", v0);
while(1){
printf("%d\n", v1);
int v4 = (v0)+(v1);
v0 = v1;
v1 = v4;
v2 = (v2)+(1);
if (v2>v3) {
break;
}
}
return 0;
}
```
*wow no indendts! what are you, human?*

---

## How to run:
```
cargo run < input/fib.txt
clang output/main.c -o output/main
./output/main

```