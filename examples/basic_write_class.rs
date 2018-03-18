extern crate jbcrs;

use std::io::{stdout, Write};
use jbcrs::basic::*;

fn main() {
    // construct a new constant pool
    let mut constant_pool = Pool::new();

    // Push the name of the class to the pool.
    let name = constant_pool.push_class("BasicExample".to_owned()).unwrap();
    let super_name = constant_pool
        .push_class("java/lang/Object".to_owned())
        .unwrap();

    let mut interfaces = Vec::new();
    interfaces.push(
        constant_pool
            .push_class("java/util/Runnable".to_owned())
            .unwrap(),
    );

    let mut fields = Vec::new();
    fields.push(new_counter_field(&mut constant_pool));

    let mut methods = Vec::new();
    methods.push(new_run_method(&mut constant_pool));

    let mut attributes = Vec::new();

    // Push a SourceFile attribute
    let src = constant_pool
        .push_utf8("BasicExample.java".to_owned())
        .unwrap();
    attributes.push(Attribute::SourceFile(src));

    // create the class
    let class = Class {
        major_version: 0x35,
        minor_version: 0x00,

        access_flags: AccessFlags::PUBLIC | AccessFlags::SUPER,
        name,
        super_name,
        interfaces,

        fields,
        methods,
        attributes,
    };

    // write bytes to stdout
    let bytes = write(&constant_pool, &class).expect("could not write bytes");
    stdout().write_all(&bytes).unwrap();
}

fn new_counter_field(constant_pool: &mut Pool) -> Field {
    let name = constant_pool.push_utf8("counter".to_owned()).unwrap();
    let desc = constant_pool.push_utf8("I".to_owned()).unwrap();

    Field {
        access_flags: AccessFlags::PRIVATE,
        name,
        desc,
        attributes: Vec::new(),
    }
}

/// Implements a run method which adds 5 to the counter field.
fn new_run_method(constant_pool: &mut Pool) -> Method {
    let name = constant_pool.push_utf8("run".to_owned()).unwrap();
    let desc = constant_pool.push_utf8("()V".to_owned()).unwrap();

    let mut attributes = Vec::new();

    // add the Code attribute
    let mut instructions = Vec::new();

    // access the counter field

    let counter_class = constant_pool.push_class("BasicExample".to_owned()).unwrap();

    let counter_name = constant_pool.push_utf8("counter".to_owned()).unwrap();
    let counter_desc = constant_pool.push_utf8("I".to_owned()).unwrap();
    let counter_name_and_type = constant_pool
        .push(Item::NameAndType {
            name: counter_name,
            desc: counter_desc,
        })
        .unwrap();

    let counter_field = constant_pool
        .push(Item::FieldRef {
            class: counter_class,
            name_and_type: counter_name_and_type,
        })
        .unwrap();

    // indices have to be sorted from low to high
    instructions.push(Some(Instruction::GetField(counter_field)));
    instructions.push(Some(Instruction::BIPush(5)));
    instructions.push(Some(Instruction::IAdd));
    instructions.push(Some(Instruction::PutField(counter_field)));
    instructions.push(Some(Instruction::Return));

    attributes.push(Attribute::Code {
        max_locals: 0,
        max_stack: 2,
        instructions,
        exceptions: Vec::new(),
        attributes: Vec::new(),
    });

    Method {
        access_flags: AccessFlags::PUBLIC,
        name,
        desc,
        attributes,
    }
}
