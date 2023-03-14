import std.stdio;

extern (C)
{
    void vol_thread_start();
    void start_rs();
    int get_thread_status();
    // void test_rs();

}
void main()
{
    vol_thread_start();
    start_rs();
    // test_rs();
}
