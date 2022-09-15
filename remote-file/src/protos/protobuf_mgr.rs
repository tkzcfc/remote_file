

use std::{collections::HashMap, any::Any};
use protobuf::Message;
use super::base;


pub type MsgParseCall = fn(&Vec<u8>)-> Option<Box<dyn Any>>;
pub struct ProtobufMgr{
    pub msg_parse_map :HashMap<String, MsgParseCall>,
}

static mut EL: *mut ProtobufMgr = 0 as *mut _;

impl ProtobufMgr {
    pub fn instance() -> &'static ProtobufMgr {
        unsafe {
            if EL == 0 as *mut _ {
                let mgr = ProtobufMgr {
                    msg_parse_map: HashMap::new()
                };
                EL = Box::into_raw(Box::new(mgr));
                (*EL).register_all();
            }
            &*EL
        }
    }

    // 注册消息解码回调
    pub fn register(&mut self, msg_name: String, call: MsgParseCall) {
        self.msg_parse_map.insert(msg_name, call).unwrap();
    }

    // 注册所有消息解码
    pub fn register_all(&mut self) {
        // 注册Person消息解析函数
        let msg_name = String::from(base::Person::descriptor_static().full_name());

        self.register(msg_name, |data: &Vec<u8>|->Option<Box<dyn Any>> {
            if let Ok(p) = base::Person::parse_from_bytes(data) {
                return  Option::Some(Box::new(p));
            }
            return  Option::None;
        });
    }

    // 消息解码
    pub fn decode(&mut self, msg_name: &String, data: &Vec<u8>)-> Option<Box<dyn Any>> {
        let call = self.msg_parse_map.get(msg_name)?;
        call(data)
    }
}

