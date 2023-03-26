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

fn par_match(open:char, close:char)->bool{
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn par_check02(par:&str)->bool{
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance{
        let c = char_list[index];

        if '(' == c || '[' == c || '{' ==c {
            stack.push(c);
        }else{
            if stack.is_empty(){
                balance =false;
            }else{
                let top = stack.pop().unwrap();
                if !par_match(top,c){
                    balance = false;
                }
            }
        }
        index +=1;
    }
    balance && stack.is_empty()
}

// that is the char is match ([{ , push in to a Stack, if not match, I want to 
// check if is_empty, and if not empty, that means something I want to pop from stack
// must match the char of the rest

fn main(){
    let sa = "(){}[]";
    let sb = "(){)[]";
    let res1 = par_check02(sa);
    let res2 = par_check02(sb);
    println!("sa balance: {res1}, sa balance: {res2}");
}