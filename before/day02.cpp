#include <iostream>
using namespace std;
#include <string>
// chater2 literals 
// sg1 this 
//thit 不能用于static 方法
class Test {
public:
	string str;
	static string say_hello() {
		//this->str = "hello world"; 不能这样做
		return "hello world";
	}
};
int main(){
	// 我不知道shared_ptr 是什么,但是我想尝鲜
	
	std::shared_ptr<Test> ptr = make_shared<Test>();
	cout << ptr->say_hello()<< endl;
	cout << Test::say_hello() << endl;//调用静态使用::
}
