#[derive(Debug)]
struct Queue<T>{
    cap:usize,
    data:Vec<T>,
}

impl<T> Queue<T>{
    fn new(size:usize)->Self{
        Queue{
            data:Vec::with_capacity(size),
            cap:size,
        }
    }

    fn enqueue(&mut self, val:T)->Result<(),String>{
        if Self::size(&self) == self.cap{
            return Err("no more capacity".to_string());
        }
        self.data.insert(0,val);
        Ok(())// Ok is alias for Result which is enum type
    }

    fn dequeue(&mut self)->Option<T>{
        if Self::size(&self) > 0{
            self.data.pop()
        }else{
            None// None is alias of Option which is enum type
        }
    }

    fn is_empty(&self)->bool{
        0 == Self::size(&self)
    }

    fn size(&self)->usize {
        self.data.len()
    }
}

fn hot_potato(names:Vec<&str>,num:usize)->&str{
    let mut q = Queue::new(names.len());
    for name in names{
        let _rm = q.enqueue(name);
    }

    while q.size() > 1{
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        let _rm = q.dequeue();
    }
    q.dequeue().unwrap()
}

fn main(){
    let name = vec!["Shieber","David","Susan","Jane","Kew","Brad"];
    let rem  = hot_potato(name,8);
    println!("the left person: {rem}");
}