// alg_any_of.cpp 当指定元素范围中至少有一个元素满足条件时返回 true。
// compile with: g++
#include <list>
#include <algorithm>
#include <iostream>
int main()
{
    using namespace std;
    list<int> li{51, 41, 11, 21, 20};
    cout << "li = ( ";
    for (auto const &el : li)
        cout << el << " ";
    cout << ")" << endl;
    // Check if there's an even element in li.
    auto is_even = [](int const elem)
    { return !(elem % 2); };
    if (any_of(li.begin(), li.end(), is_even))
        cout << "There's an even element in li.\n";
    else
        cout << "There are no even elements in li.\n";
}
// template <class InputIterator, class UnaryPredicate>
// bool any_of(
//     InputIterator first,
//     InputIterator last,
//     UnaryPredicate pred);
// template <class ExecutionPolicy, class ForwardIterator, class UnaryPredicate>
// bool any_of(
//     ExecutionPolicy &&exec,
//     ForwardIterator first,
//     ForwardIterator last,
//     UnaryPredicate pred);
