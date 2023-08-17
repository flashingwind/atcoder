#include <stdio.h>
int    main(void)
{
    char buf[12];//0は使わない、1-11文字目、null文字
	char *s=buf;
	s++;

    scanf("%s",s);

    int a;
    int b;

    scanf("%d%d",&a,&b);

    // swap
    printf("%c %c\n",s[a],s[b]);
    char tmp = s[b];
    s[b]=s[a];
    s[a]=tmp;

    printf("%s\n", s);
    return (0);
}
