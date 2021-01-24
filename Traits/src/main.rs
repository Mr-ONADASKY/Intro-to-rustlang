struct A {
    a: String,
}

impl Drop for A {
    fn drop(&mut self) {
        println!("dropped {}", self.a);
    }
}

fn main() {
    let a = A {
        a: String::from("A"),
    };
    {
        let b = A {
            a: String::from("B"),
        };

        {
            let c = A {
                a: String::from("C"),
            };

            println!("Leaving inner scope 2");
        }

        println!("Leaving inner scope 1");
    }
    drop(a);
    println!("Program ending");
}

// use std::ops;

// struct A;
// struct B;
// #[derive(Debug)]
// struct AB;
// #[derive(Debug)]
// struct BA;

// impl ops::Add<B> for A {
//     type Output = AB;

//     fn add(self, _rhs: B) -> AB {
//         AB
//     }
// }

// impl ops::Add<A> for B {
//     type Output = BA;

//     fn add(self, _rhs: A) -> BA {
//         BA
//     }
// }

// fn main() {
//     println!("{:?}", A + B);
//     println!("{:?}", B + A);
//     // println!("{:?}", B + B);
// }

// trait Shape {
//     fn area(&self) -> u32;
// }

// struct Rectangle {
//     x: u32,
//     y: u32,
// }

// struct Circle {
//     radius: f64,
// }

// impl Shape for Rectangle {
//     fn area(&self) -> u32 {
//         self.x * self.y
//     }
// }

// impl Shape for Circle {
//     fn area(&self) -> u32 {
//         (3.14159265 * self.radius * self.radius) as u32
//     }
// }

// #[derive(Debug, Clone, Copy)]
// struct A(i32);
// struct B(f32);

// fn main() {
//     let c = Circle { radius: 100.132 };
//     let r = Rectangle { x: 30, y: 20 };
//     println!("{} {}", c.area(), r.area());

//     let a = A(32);
//     let b = B(12.13);
//     let c = a;
//     println!("{:?}", a);
// }
