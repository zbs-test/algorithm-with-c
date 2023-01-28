// #!看注释
#include <iostream>

/*模板种类---泛型为了函数和类(对象)
函数模板    Function
类模板      Class
操作符重载  Operator
*/
// 尽量使用typename
template <typename T>
inline T Max(const T &a, const T &b)
{
    // 真正调用才能做出函数的地址,模板调用两步:1.检查模板定义语法，2.检查调用函数是否合法
    return a > b ? a : b;
}
// 函数模板重载，重载的条件是参数不完全一致，返回值不一致，一般是这两种
template <typename T>
inline const T &Max(const T &a, const T &b, const T &c) // const 接近类型名(这里为T)叫做常类型，接近指针名为常指针，除开const接近指针的情况，const 在类型的前后效果等价
{
    const T& tmp = a > b ? a : b;   //保证类型于函数返回值类型一致的有效办法就是模仿返回值类型
    const T& max = tmp > c ? tmp : c;
    return max;
}
int main(int argc, char const *argv[])
{
    using namespace std;
    int a = Max(1, 2);//函数优先调用本地普通函数，模板函数调用有时间开销
    cout << a << endl;

    int i = 7, j = 30;
    _printf_p("%d\n", Max(i, j)); // 这是什么函数？？

    cout << Max(static_cast<double>(1), 2.0) << endl; // 参数不匹配了怎么办?,使用static_cast<typename>(conv_value)
    cout << Max<double>(1, 2.0) << endl;

    cout << Max(1, 2, 9) << endl;

    return 0;
}
