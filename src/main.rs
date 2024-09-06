
trait Animal {
    fn name(&self)->&'static str;
    fn talk (&self){
        println!("{} cannot talk ",self.name())
    }
}

struct Human {
    isim:&'static str
}
struct  Cat {
    isim:&'static str
}
impl Animal for Human {
    fn name(&self) -> &'static str{
        self.isim
    }

    fn talk (&self) {
        println!("{} says hello",self.name())// Eğer fn talk komutunu yorum satırı içine alırsan kadir cannot talk yazıcak
    }
}

impl Animal for Cat{
    fn name(&self)->&'static str {
        self.isim
    }
    fn talk (&self) {
        println!("{} says weeow ",self.name());
    }
}

fn traits(){
    let h = Human{isim:"Kadir"};
    h.talk();

    let y = Cat{isim:"Camgöz"};
    y.talk();
}

fn main() {
    traits();
}