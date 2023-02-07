pub trait  Show_Profile {
    fn  print1(&self);
}

pub struct  Profile_Student{
    pub name : String,
    pub age : i32,
}

impl Show_Profile for Profile_Student {    
      fn  print1(&self) {
           println!("NAME=>{}: AGE=>{}", self.name, self.age);
      }
}
