static int getXOfLong(long a){
    return (int)(a >> 32);
}
int getYOfLong(long a){
    return (int)(a);
}
long packInts(int x, int y){
    return (long)x << 32 | y & 0xFFFFFFFFL;
}