// alg_adj_fnd.cpp  搜索相等或满足指定条件的两个相邻元素。test01 STL上道
// compile with: g++
#include <list>
#include <algorithm>
#include <iostream>
// Returns whether second element is twice the first
bool twice(int elem1, int elem2)
{
    return elem1 * 2 == elem2;
}
int main()
{
    using namespace std;
    list<int> L;
    list<int>::iterator Iter;
    list<int>::iterator result1, result2;
    L.push_back(50);
    L.push_back(40);
    L.push_back(10);
    L.push_back(20);
    L.push_back(20);
    cout << "L = ( ";
    for (Iter = L.begin(); Iter != L.end(); Iter++)
        cout << *Iter << " ";
    cout << ")" << endl;
    // 邻接查找,默认为相等的两个元素
    result1 = adjacent_find(L.begin(), L.end());
    if (result1 == L.end())
        cout << "There are not two adjacent elements that are equal."
             << endl;
    else
        cout << "There are two adjacent elements that are equal.\n"
             << "They have a value of "
             << *(result1) << "." << endl;
    // 邻接查找，逻辑为上面定义的bool函数
    result2 = adjacent_find(L.begin(), L.end(), twice);
    if (result2 == L.end())
        cout << "There are not two adjacent elements where the "
             << "second is twice the first." << endl;
    else
    {
        cout << "There are two adjacent elements where "
             << "the second is twice the first.\n"
             << "They have values of " << *(result2++)
             << " & " << *result2 << "." << endl;
    }
}
// template<class ForwardIterator>
// ForwardIterator adjacent_find(
//  ForwardIterator first,
//  ForwardIterator last);
// template<class ForwardIterator , class BinaryPredicate>
// ForwardIterator adjacent_find(
//  ForwardIterator first,
//  ForwardIterator last,
//  BinaryPredicate pred);
// template<class ExecutionPolicy, class ForwardIterator>
// ForwardIterator adjacent_find(
//  ExecutionPolicy&& exec,
//  ForwardIterator first,
//  ForwardIterator last);
// template<class ExecutionPolicy, class ForwardIterator, class
// BinaryPredicate>
// ForwardIterator adjacent_find(
//  ExecutionPolicy&& exec,//执行逻辑
//  ForwardIterator first,//第一个元素位置处的前向迭代器
//  ForwardIterator last,//最后一个元素之后下一个元素位置处的前向迭代器
//  BinaryPredicate pred);//二元谓词逻辑


