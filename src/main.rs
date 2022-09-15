use std::any::Any;
use std::collections::HashMap;
use protobuf::Message;
use protobuf::CodedOutputStream;
use paste::paste;

extern crate proc_macro;
// use proc_macro::TokenStream;

mod protos;

// macro_rules! sendProtoBuf {
//     ($valName: expr) => {
//         let mut data = Vec::new();
//         let mut os = CodedOutputStream::new(&mut data);
    
//         if let Err(err) = $valName.write_to(&mut os) 
//         {
//             println!("write proto buf error: {:?}", err);
//         }
//         else 
//         {
//             os.flush().unwrap();
//             println!("data size: {}", data.len());
//         }
//     };
// }


use protos::Test::*;


macro_rules! auto_register {
    ($msg_type: ty  ) => {
        println!("msg name: {}", <$msg_type>::descriptor_static().full_name());
        paste! {
            println!("msg id: {}", [<$msg_type _MsgId>]::Id as u32);
        }
    }

    // Person_MsgId
    // concat_idents!($msg_type, _MsgId)

}

fn test_person() {
    println!("person_test==========>>");
}

// #![feature(concat_idents)]
macro_rules! call_me {
    ($msg_type:tt   ) => {
        paste! {[<test_ $msg_type>]();}
        // contact_idents!(test_, $msg_type)();
    }

    // Person_MsgId
    // concat_idents!($msg_type, _MsgId)

}

// [<get_ $field>]


// macro_rules! test {
//     ($x:ident) => ({
//         let z = concat_idents!(hello_, $x);
//         z();
//     })
// }

// fn hello_world() {  }


fn main() {
    // call_me!(person);
    auto_register!(Person);

    // stringify!(protos::Test::Person);
    
    // let mut codegen = protobuf_codegen_pure::Codegen::new();
    // codegen.out_dir("src/protos")
    //     .include("src/protos/src/")
    //     .inputs(&["src/protos/src/Test.proto"])
    //     .run()
    //     .expect("compile protoc failed.");


    // type MsgParseCall = fn(&Vec<u8>)-> Option<Box<dyn Any>>;

    // // 消息结构解析映射
    // let mut map:HashMap<String, MsgParseCall > = HashMap::new();

    // // 注册Person消息解析函数
    // let msg_name = String::from(protos::Test::Person::descriptor_static().full_name());
    // map.insert(msg_name, |data: &Vec<u8>|->Option<Box<dyn Any>> {
    //     if let Ok(p) = protos::Test::Person::parse_from_bytes(data) {
    //         return  Option::Some(Box::new(p));
    //     }
    //     return  Option::None;
    // });

    // // 获取消息Person解析函数
    // let it = map.get(&String::from(protos::Test::Person::descriptor_static().full_name()));
    // if let Option::Some(call) = it {
        
    //     // 模拟收到的二进制数据
    //     let mut person = protos::Test::Person::new();
    //     person.age = 100;
    //     person.name = String::from("hahahah");
    //     person.id = 147258;
        
    //     // 动态解析
    //     let mut data = Vec::new();
    //     let mut os = CodedOutputStream::new(&mut data);
    //     person.write_to(&mut os).unwrap();
    //     os.flush().unwrap();

    //     println!("data = {}", data.len());

    //     // 解析成功
    //     if let Option::Some(msg) = call(&data) {
    //         match msg.downcast_ref::<protos::Test::Person>() {
    //             Some(p)=>{
    //                 println!("person name = {}", p.name);
    //                 println!("person age = {}", p.age);
    //                 println!("person id = {}", p.id);
    //             },
    //             None=> {
    //                 println!("not person");
    //             }
    //         }
    //     }
    // }
}
