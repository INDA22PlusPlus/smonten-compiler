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