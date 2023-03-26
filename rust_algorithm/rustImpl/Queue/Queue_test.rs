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

fn main(){
    let mut q = Queue::new(3);
    let _r1 = q.enqueue(1);// if ok, will return Ok(()) and like unit or void
    let _r2 = q.enqueue(2);// if ok, will return Ok(()) and like unit or void
    let _r3 = q.enqueue(3);// if ok, will return Ok(()) and like unit or void
    if let Err(error) = q.enqueue(4){
        println!("enqueue error: {error}");
    }

    if let Some(data) = q.dequeue(){
        println!("the head of q: {data}");// from left to right, right is head
    }else{
        println!("q is empty");
    }

    println!("size: {}, empty: {}",q.size(), q.is_empty());
    println!("q: {:?}", q);
}