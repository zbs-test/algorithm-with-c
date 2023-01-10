#include <iostream>
using namespace std;
struct S {
private:
	int x;
public:
	/*S() {
		this->x = 0;
	}*/
	//一个重载+符号的函数,call by: obj.operator+(other_obj), 返回obj,即调用的对象
	S& operator+(const S& other) {

		x +=  other.x;
		return *this;
	}
	
	int operator-(const S& other) {
		return  other.x;
	}
	int getX() {
		return this->x;
	}

};
struct Under_S :public S {

};
class Father {
public:
	int hey_I_am_public;
	Father(){}
	Father(int a,int b):a(a),b(b) {
		
	}
private:
		int a, b;
protected:
	int what_if_be_public;
};
class Son :public Father {
public:
	//Son(){}
};
int main() {
	struct S* s = new S();//by default
	struct S* ss = new S();
	s->operator+(*ss);//调用函数的艺术
	cout <<  "hey call operator- :"<<s->operator-(*ss) << endl;
	cout <<"hey this is s "<< s->getX() << endl;
	struct S* father = new Under_S();// 父类指针
	cout << "hey this is father " << father->getX() << endl;
	//struct Under_S* son = new S();  I can't take control of father
	class Son* song = new Son();//当使用默认的构造函数，Father必须含有默认的构造函数
	//song->what_if_be_public = 1;// 即使是公有继承，protected private 依然不会继承为public 
	cout << song->hey_I_am_public << endl;
	delete s,ss,father,song;
	
}
