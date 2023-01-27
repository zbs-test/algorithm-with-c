// alg_all_of.cpp 当给定范围中的每个元素均满足条件时返回 true。
// compile with: g++
#include <list>
#include <algorithm>
#include <iostream>
int main()
{
    using namespace std;
    list<int> li{50, 40, 10, 20, 20};
    list<int>::iterator iter;
    cout << "li = ( ";
    for (iter = li.begin(); iter != li.end(); iter++)
        cout << *iter << " ";
    cout << ")" << endl;
    // Check if all elements in li are even.
    auto is_even = [](int elem)
    { return !(elem % 2); }; // !(..) 等价于 (...=0)
    if (all_of(li.begin(), li.end(), is_even))
        cout << "All the elements are even numbers.\n";
    else
        cout << "Not all the elements are even numbers.\n";
}
// template <class InputIterator, class UnaryPredicate>
// bool all_of(
//     InputIterator first,
//     InputIterator last,
//     UnaryPredicate pred);
// template <class ExecutionPolicy, class ForwardIterator, class UnaryPredicate>
// bool all_of(
//     ExecutionPolicy &&exec,//执行策略是什么，一般使用第一个函数
//     ForwardIterator first,
//     ForwardIterator last,
//     UnaryPredicate pred);
