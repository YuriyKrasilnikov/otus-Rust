// Decorator

use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Base {
    text: String,
}

trait Print {
    fn print(&self);
}

impl Print for Base {
    fn print(&self) {
        println!("{}", self.text);
    }
}

struct GenDebugDecorator<T: Debug + Print>(T);

impl<Base: Debug + Print> GenDebugDecorator<Base> {
    fn new(base: Base) -> Self {
        GenDebugDecorator(base)
    }
}

impl<Base: Debug + Print> Print for GenDebugDecorator<Base> {
    fn print(&self) {
        println!("{:?}", self.0);
        self.0.print();
    }
}

trait DynPrint: Debug {
    fn dynprint(&self);
}

impl DynPrint for Base {
    fn dynprint(&self) {
        print!("{}", self.text);
    }
}

struct DynDebugDecorator(Box<dyn DynPrint>);

impl DynDebugDecorator {
    fn new(base: Box<dyn DynPrint>) -> Self {
        DynDebugDecorator(base)
    }

    fn dynprint(&self) {
        println!("{:?}", self.0);
        self.0.dynprint();
    }
}

fn main() {
    let t = Base {
        text: "Hello, World!".to_string(),
    };
    println!("Print without decorators");
    t.clone().print();

    println!("---//---");
    println!("Print with generic decorators");
    let gt = GenDebugDecorator::new(t.clone());
    gt.print();

    println!("---//---");
    println!("Print with dynamic decorators");
    let dt = DynDebugDecorator::new(Box::new(t.clone()));
    dt.dynprint();
}
