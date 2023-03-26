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

fn pal_check01(pal:&str)->bool{
    let mut d = Deque::new(pal.len());
    for c in pal.chars(){
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.size() > 1 && is_pal{
        let head = d.remove_front();
        let tail = d.remove_rear();
        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}

fn main(){
    let pal = "rustsur";
    let is_pal = pal_check01(pal);
    println!("{pal}: {is_pal}");
}