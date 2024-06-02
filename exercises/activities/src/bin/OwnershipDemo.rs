type Id = u32;

#[derive(Debug)]
struct Handle(Id);

// ~ destructor
impl Drop for Handle {
    fn drop(&mut self) {
        println!("self: {:?} - customized message - handle {} dropped!", self, self.0);
    }
}

fn main() {
    // bottom to top approach 
    // LIFO - Last In First Out
    // STACK - LIFO
    let handle_0 = Handle(0); // Handle(0)'s value now owned by var handle_0
    let handle_1 = Handle(1);
    let handle_2 = Handle(2);
    Handle(9); // Handle(9) is the first one exposed, it's scope gets over in main function itself, Handle(9) will be destroyed
    let handle_8 = create_handle1(); // there after Handle(3,4,7) ((5,6,...88)[later]) straight in function as they are directly exposed, not reused by a var, and there scope get's over
    // returns 8 back here in a variable, ownership in main variable
    Handle(10); // once all exposed in create_handle1 are removed, scope will get back to Handle(9) in main but will store returned value from create_handle1 to var8
    let _ = create_handle3(); // there after Handle(333,444,777) ((555,666,...888)[later]) straight in function as they are directly exposed, not reused by a var, and there scope get's over
    // returns 888 back here in a '_' variable, ownership in main variable and will be destroyed here itself instantly, since using '_' var
    Handle(11); // once all exposed in create_handle3 are removed, scope will get back to Handle(10) in main also will remove above '_'var
    // once the scope reaches end of code, it will start removing the variables in LIFO style
}

fn create_handle1() -> Handle {
    Handle(3);
    Handle(4);
    let handle_5 = Handle(5);
    let handle_6 = Handle(6);
    let handle_88 = create_handle2();
    Handle(7);
    Handle(8)
}

fn create_handle2() -> Handle {
    Handle(33);
    Handle(44);
    let handle_5 = Handle(55);
    let handle_6 = Handle(66);
    Handle(77);
    Handle(88)
}

fn create_handle3() -> Handle {
    Handle(333);
    Handle(444);
    let handle_5 = Handle(555);
    let handle_6 = Handle(666);
    Handle(777);
    Handle(888)
}
