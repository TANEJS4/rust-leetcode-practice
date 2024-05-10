#[allow(dead_code)]
pub struct MinStack {
    stack: Vec<i32>,
    min: Option<i32>, 
    lastmin : bool
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]

impl MinStack {

    fn new() -> Self {
        MinStack{
            stack: vec![],
            min : None, 
            lastmin : false
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min == None || self.min > Some(val){
            self.min =  Some(val);
            self.lastmin =  true;
        } else {
            self.lastmin = false;
        }

    }
    
    fn pop(&mut self) {
        if self.stack.is_empty(){
            panic!("Not allowed to pop empty Stack"); 
        }
        self.stack.pop();
        if self.lastmin {
            self.lastmin = false;
        }
        self.min = self.get_min_helper();
    }
    
    fn top(&self) -> i32 {
        if self.stack.is_empty(){
            panic!("Empty Stack"); 
        }
        *self.stack.last().unwrap()
    }
    fn get_min_helper(&mut self) -> Option<i32> {
        self.min  =  self.stack.iter().min().copied();
        
        return self.min
    }
    fn get_min(&mut self) -> i32 {
        self.min  =  self.stack.iter().min().copied();
        
        return self.min.unwrap()
    }
}
