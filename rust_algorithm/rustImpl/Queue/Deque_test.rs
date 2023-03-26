// 双端队列
#[derive(Debug)]
struct Deque<T>{
    cap:usize,
    data:Vec<T>,
}

impl<T> Deque<T> {
    fn new(cap:usize)->Self{
        Deque{
            cap:cap,
            data:Vec::with_capacity(cap),
        }
    }

    fn add_front(&mut self, val:T)->Result<(),String>{
        if Self::size(&self) == self.cap{
            return Err("no more capacity".to_string());
        }

        self.data.push(val);
        Ok(())
    }

    fn add_rear(&mut self, val:T)->Result<(),String>{
        if Self::size(&self) == self.cap{
            return Err("no more capaticy".to_string());
        }

        self.data.insert(0,val);
        Ok(())
    }

    fn remove_front(&mut self)->Option<T>{
        if Self::size(&self) > 0{
            self.data.pop()
        }else {
            None
        }
    }

    fn remove_rear(&mut self)->Option<T>{
        if Self::size(&self)>0{
            Some(self.data.remove(0))
        }else{
            None
        }
    }

    fn is_empty(&self)->bool{
        0 == Self::size(&self)
    }

    fn size(&self)->usize{
        self.data.len()
    }
}

fn main(){
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_rear(3);
    let _r4 = d.add_rear(4);
    if let Err(error) = d.add_front(5) {
        println!("add_front error:{error}");
    }

    if let Some(data) = d.remove_rear(){
        println!("{data}");
    }else{
        println!("d is empty");
    }
    println!("size: {}, is_empty: {}", d.size(),d.is_empty());
    println!("d: {:?}",d);
}
