mod quiz_2_q4;
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(5);
    println!("Square dimension is {:#?}", square);

    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    println!("r after set_width method: {:?}", r);
    Rectangle::set_width(&mut r, 3);
    println!("r after set_width function call: {:?}", r);

    let rect4 = Rectangle {
        width: 0,
        height: 0,
    };
    println!("rect4 area: {}", rect4.area());
    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect4.max(&other_rect);
    println!("max_rect: {:?}", max_rect);
    // println!("{:?}", rect4); // this will fail as rect4 loses ownership at rect4.max(self)

    let mut rect5 = Rectangle {
        width: 0,
        height: 0,
    };
    rect5.set_width(0);

    let rect5_ref = &rect5;
    // rect5_ref.set_width(2); // will not be OK

    let rect5_ref_mut = &mut rect5;
    rect5_ref_mut.set_width(2); // will be OK with &mut
    println!("rect5: {:?}", rect5);

    let mut rect6 = Rectangle {
        width: 0,
        height: 0,
    };
    rect6.set_to_max(&other_rect);
    quiz_2_q4::main();
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
    fn max(self, other: &Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    fn set_to_max(&mut self, other: &Rectangle) {
        let max = self.max(other); // will not work unless Copy, Clone traits in derived for Rectangle struct
        *self = max;
    }
}
