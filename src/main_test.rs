// use std::{io::{self, Write}, u8};




// fn print() {
//     let str = "i am string  dsdf ";
//     println!("[str]: {}", str);

//     let value:f32 = 100.123456;
//     println!("[value]:{}", value);    

//     print!("this ");
//     print!("will ");
//     print!("be ");
//     print!("on ");
//     print!("the ");
//     print!("same ");
//     print!("line ");
//     print!("\n");
//     io::stdout().flush().unwrap();
// }

// fn var_test() {
//     // 报错
//     // const a: i32 = 123;
//     // let a = 456;

//     // 重新绑定，可以
//     // let a: i32 = 123;
//     // let a = "456";

//     let s = "123456";
//     let s = s.len();
//     println!("s = {}", s);    

//     let value = b'b';
//     println!("value = {}", value);
// }

// fn tup_test() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     // tup.0 等于 500
//     // tup.1 等于 6.4
//     // tup.2 等于 1
//     let (x, y, z) = tup;
//     // y 等于 6.4    
// }

// fn array_test() {
//     let a = [1, 2, 3, 4, 5];
//     // a 是一个长度为 5 的整型数组

//     let b = ["January", "February", "March"];
//     // b 是一个长度为 3 的字符串数组

//     let c: [i32; 5] = [1, 2, 3, 4, 5];
//     // c 是一个长度为 5 的 i32 数组

//     let d = [3; 5];
//     // 等同于 let d = [3, 3, 3, 3, 3];

//     let first = a[0];
//     let second = a[1];
//     // 数组访问

//     // a[0] = 123; // 错误：数组 a 不可变
//     let mut a = [1, 2, 3];
//     a[0] = 4; // 正确
// }

// fn fn_block_test_1() {
    
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("x 的值为 : {}", x);
//     println!("y 的值为 : {}", y);

//     // x 的值为 : 5
//     // y 的值为 : 4
// }

// fn fn_block_test_2() {
//     fn five() -> i32 {
//         5
//     }
//     println!("five() 的值为: {}", five());
// }

// fn sanyuan_test() {
//     let a = 3;
//     let number = if a > 0 { 1 } else { -1 };
//     println!("number 为 {}", number);
// }

// fn for_test_1() {
//     let a = [10, 20, 30, 40, 50];
//     for i in a.iter() {
//         println!("值为 : {}", i);
//     }
// }

// fn for_test2() {
//     let a = [10, 20, 30, 40, 50];
//     let count = a.len();
//     for i in 0..count {
//         println!("a[{}] = {}", i, a[i]);
//     }
// }

// fn loop_test() {
//     let s = ['R', 'U', 'N', 'O', 'O', 'B'];
//     let mut i = 0;
//     let location: i32 = loop {
//         if(i >= s.len() - 1) {
//             break -1;
//         }
//         let ch = s[i];
//         if ch == 'O' {
//             break i as i32;
//         }
//         i += 1;
//     };
//     println!(" \'O\' 的索引为 {}", location);
// }

// fn mem_mgr_move_test1() {
//     let s1 = String::from("hello");
//     let s2 = s1; 
//     // println!("{}, world!", s1); // 错误！s1 已经失效
// }

// fn mem_mgr_clone_test1() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn mem_mgr_test1() {
//     let s = String::from("hello");
//     // s 被声明有效

//     takes_ownership(s);
//     // s 的值被当作参数传入函数
//     // 所以可以当作 s 已经被移动，从这里开始已经无效

//     // println!("s = {}", s); // 此处错误,s已经被移动了，所有权失效

//     let x = 5;
//     // x 被声明有效

//     makes_copy(x);
//     // x 的值被当作参数传入函数
//     // 但 x 是基本类型，依然有效
//     // 在这里依然可以使用 x 却不能使用 s

// } // 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放


// fn takes_ownership(some_string: String) {
//     // 一个 String 参数 some_string 传入，有效
//     println!("{}", some_string);
// } // 函数结束, 参数 some_string 在这里释放

// fn makes_copy(some_integer: i32) {
//     // 一个 i32 参数 some_integer 传入，有效
//     println!("{}", some_integer);
// } // 函数结束, 参数 some_integer 是基本类型, 无需释放


// fn test() {
//     let s = String::from("zcx");
//     let ss = trans(s);
//     println!("s: {}", s);
//     println!("s: {}", ss);

//     let sp = &s[0..1];

// }

// fn trans(str: String) -> &String
// {
//     struct Color(u8, u8, u8);
//     struct Size { width: u32, height: u32 };

//     let red = Color(255, 0, 0);
//     let size = Size {width: 100, height: 1000};
//     size.width = 2001;

//     size = Size {width: 10, height : 200};

    
//     return &str;
// }

use std::{vec, cell::RefCell, mem};

fn max(array: &[i32]) -> Result<&i32, &str> {
    if(array.is_empty()) {
        return Result::Err("bad array");
    }

    let mut value = &array[0];
    for i in 1 .. array.len() - 1 {
        if(value < &array[i]) {
            value = &array[i];
        }
    }
    return Result::Ok(value);
}



fn max_t<T: CompareTrait>(array: &[T]) -> Result<&T, &str> {
    if(array.is_empty()) {
        return Result::Err("bad array");
    }

    let mut value = &array[0];
    for i in 1 .. array.len() - 1 {
        if(value.compare(&array[i]) < 0) {
            value = &array[i];
        }
    }
    return Result::Ok(value);
}

trait CompareTrait {
    fn compare(&self, other: &Self)->i8 {
        return 0;
    }
}

impl CompareTrait for i32 {
    fn compare(&self, other: &Self)->i8 {
        if &self > &other {
            return 1;
        }
        else if &self == &other {
            return 0;
        }
        return -1;
    }
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if(s1.len() > s2.len()) {
        return s1;
    }
    return s2;
}


fn main() {
      let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，这里并没有分配新的字符串
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut ivalue1 = 100;
    let mut ivalue2 = 100;
    let ref mut refValue = ivalue1;

    *refValue = 123456;

    println!("refValue = {}", *refValue);
    println!("value1 = {}", ivalue1);

    
    // let ref mut v3 = v1;
    // *v3 = 300;
    // println!("v3 = {}", *v3);


    // let mut value = 54;
    // let ref mut refvalue = value;
    // *refvalue = 100;

    // let value = value;
    // println!("value {}", value);
    // println!("refvalue {}", refvalue);
    
    
    // print();
    // var_test();

    let array = [1, 9, 4, 3, 456, 94];
    let value = max(&array);

    if let Result::Ok(v) = value {
        println!("function[max] result {}", v);
    }
    else {
        println!("function[max] error");
    };
    

    let maxValue = max_t(&array);
    match maxValue {
        Result::Ok(v) => {
            println!("function[max_t] result {}", v);
        }
        Result::Err(v)=> {
            println!("function[max_t] error");
        }
    }

    let mut longstr: &str;
    {
        let s1 = "abcde";
        let s2 = "bcdefcc";
        longstr = longer(&s1, &s2);
        println!("longstr = {}", longstr);
    }
    // 可以执行，longer函数返回是直接把值copy给longstr了
    // 如果类型是String就会报错，数据在堆上，返回时不会自动copy
    // https://www.runoob.com/rust/rust-lifetime.html   最后
    println!("longstr1 = {}", longstr);


    
    // enum Book {
    //     Papery(u32),
    //     Electronic(String)
    // }
    // let book = Book::Electronic(String::from("url"));
    // if let Book::Papery(index) = book {
    //     println!("Papery {}", index);
    // } else {
    //     println!("Not papery book");
    // }
    
    // match book {
    //     Book::Papery(value) =>{
    //         println!("Papery value: {}", value);
    //     }        
    //     Book::Electronic(value) => {
    //         println!("Electronic value: {}", value);            
    //     }
    // }

    // #[derive(Debug)]
    // enum Color {
    //     Red,
    //     Blue
    // }
        
    // let cc = Color::Red;
    // println!("{:?}", cc);

    // let opt:Option<&str> = Option::None;
    // if Option::None == opt {
    //     println!("i am empty");
    // }
    // else {
    //     println!("not empty");
    // }

    
    // if let Option::None = opt {
    //     println!("i am empty  222");
    // }
    // else {
    //     println!("not empty 222");
    // }

    // fn_block_test_1();
    // fn_block_test_2();
    // loop_test();
}
