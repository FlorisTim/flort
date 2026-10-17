class Test{
    void test(){}
}

class Program{
    void main(){
        Test[] array = new Test[5];
        for (int i = 0; i < 10; i += 1){
            array[i].test();
        }
    }
}