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

fn base_converter(mut dec_num:u32,base:u32)->String{
    let digits = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];

    let mut rem_stack = Stack::new();
    while dec_num >0{
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    let mut base_str="".to_string();
    while !rem_stack.is_empty(){
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    } 
    base_str
}

fn main(){
    let bin_str = base_converter(10,2);
    let hex_str = base_converter(45,16);
    let some_str = base_converter(33,20);// 1 * 20 + 13(which is D)
    // let awful_str = base_converter(99,60); you should not do some unexpected job out of reason
    println!("{bin_str}, {hex_str}, {some_str}");
}