struct Stack<T>{
    top:usize,
    data:Vec<T>,
}

impl<T> Stack<T> {
    fn new()->Self{
        Stack{
            top:0,
            data:Vec::new()
        }
    }

    fn push(&mut self,val:T){
        self.data.push(val);
        self.top += 1;
    }

    fn pop(&mut self)->Option<T>{
        if self.top == 0{
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }
    // I dont want to get the owner of datatype T
    fn peek(&self)->Option<&T>{
        if self.top == 0{
            return None;
        }
     
        self.data.get(self.top - 1)
    }

    fn is_empty(&self)->bool{
        0==self.top
    }

    fn size(&self)->usize{
        self.top
    }
}
// for check match 
fn par_match(open:char, close:char)->bool{
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}
fn par_check03(par:&str)->bool{
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance{
        let c = char_list[index];
        if '(' == c || '[' ==c || '{' == c{
            stack.push(c);// if left , push in stack
        }
        if ')' == c|| ']' == c|| '}' ==c{// but what if is right 
            if stack.is_empty(){
                balance = false;
            }else{
                let top = stack.pop().unwrap();
                if !par_match(top,c){
                    balance = false;
                }
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}

fn main(){
    let sa = "(2+3){func }[abc]";
    let sb = "(2+3)*[a3 -1";
    let res1 = par_check03(sa);
    let res2 = par_check03(sb);
    println!("sa balance: {res1}, sb balance: {res2}");
}