use std::cmp::Ordering;
use std::ops::Deref;

type Link<T,U> = Option<Box<BST<T,U>>>;

struct BST<T,U>{
    key:Option<T>,
    val:Option<U>,
    left:Link<T,U>,
    right:Link<T,U>,
}
impl<T,U> BST<T,U>
where T:Clone + Ord + std::fmt::Debug,
            U:Clone + std::fmt::Debug{
    fn new()->Self{
        BST{
            key:None,val:None,left:None,right:None,
        }
    }

    fn is_empty(&self)->bool{
        self.key.is_none()
    }

    fn len(&self)->usize{
        self.calc_len(0)
    }

    fn calc_len(&self,mut i:usize)->usize{
        if self.key.is_none(){
            return i;
        }
        i += 1;
        if !self.left.is_none(){
            i = self.left.as_ref().unwrap().calc_len(i);
        }
        if !self.right.is_none(){
            i = self.right.as_ref().unwrap().calc_len(i);
        }
        i
    }

    fn preorder(&self){
        println!("key: {:#?},val: {:#?}",&self.key,&self.val);
        match &self.left {
             Some(node)=>node.preorder(), 
            None =>(), 
        }
        match &self.right{
            Some(node)=>node.preorder(),
            None=>(),
        }
    }

    fn inorder(&self){
        println!("key: {:#?}, val: {:#?}",&self.key,&self.val);
        match &self.left{
            Some(node)=>node.inorder(),
            None=>(),
        }
        match &self.right{
            Some(node)=>node.inorder(),
            None=>(),
        }
    }

    
}

fn main(){
    let mut bst = BST::<i32,char>::new();
    bst.insert(1,'A');
    bst.insert(2,'A');
    bst.insert(3,'A');
    bst.insert(4,'A');
    bst.insert(5,'A');

}
