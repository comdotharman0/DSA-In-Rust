pub struct Fibonacci;
impl Fibonacci{
pub fn new()->Self{
Fibonacci
}

pub fn loops(&self,mut count:u32)->usize{
let mut first = 0;
let mut second = 1;
count -= 2;
println!("{} {}",first, second);
loop {
if count==0{
break second;
}
let  new_second = first + second;
println!("{}",new_second);
first = second;
second = new_second;
count-=1;

}

}

pub fn for_loops(&self, count:u32)->usize{
let mut first = 0;
let mut second = 1;
println!("{} {}",first, second);
for _i in 0..count{
let new_second = first + second;
println!("{}",new_second);
first = second;
second = new_second;
}
second
}



pub fn while_loops(&self,mut count:u32)->usize{
let mut first = 0;
let mut second = 1;
if count>2{
count -= 1;
}else{
return count as usize;
}
println!("{} {}",first, second);
while count>0{
let new_second = first + second;
println!("{}",new_second);
first = second;
second = new_second;
count-=1;

};
second
}



pub fn recursion(&self,n:u32)->usize{
if n<2{
//println!("{}",n);
 n as usize
}else{
let higher_n=
(self.recursion(n-1)+self.recursion(n-2))
 as usize;
//println!("{}",higher_n);
higher_n
}
}

}
