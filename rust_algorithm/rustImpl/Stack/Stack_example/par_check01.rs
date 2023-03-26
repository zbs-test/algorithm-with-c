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

fn par_check01(par:&str)->bool{
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance{
        let c = char_list[index];

        if '(' == c {
            stack.push(c);
        }
        else{
            if stack.is_empty(){
                balance = false;
            }else{
                let _r = stack.pop();
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}

fn main(){
    let sa = "()(())";
    let sb = "()((())";
    let res1 = par_check01(sa);
    let res2 = par_check01(sb);
    println!("sa balance: {res1}, sb balance: {res2}");
}