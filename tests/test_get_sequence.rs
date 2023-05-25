use rust_tools::Task;

#[test]
fn test_get_sequence_01() {
    let t1 = Task::new(1);
    let t2 = Task::new(2);
    let t3 = Task::new(3);
    let t4 = Task::new(4);
    t4.borrow_mut().add_child(&t3);
    t3.borrow_mut().add_child(&t2);
    t3.borrow_mut().add_child(&t1);
    assert_eq!(t4.borrow().get_sequence().unwrap(), vec![2, 1, 3, 4]);
}

#[test]
fn test_get_sequence_02() {
    let t1 = Task::new(1);
    let t2 = Task::new(2);
    let t3 = Task::new(3);
    let t4 = Task::new(4);
    let t5 = Task::new(5);
    t4.borrow_mut().add_child(&t3);
    t3.borrow_mut().add_child(&t2);
    t3.borrow_mut().add_child(&t1);
    t2.borrow_mut().add_child(&t5);
    t1.borrow_mut().add_child(&t5);
    assert_eq!(t4.borrow().get_sequence().unwrap(), vec![5, 2, 1, 3, 4]);
}

#[test]
fn test_get_sequence_cycle_01() {
    let t1 = Task::new(1);
    let t2 = Task::new(2);
    let t3 = Task::new(3);
    let t4 = Task::new(4);
    let t5 = Task::new(5);
    t4.borrow_mut().add_child(&t3);
    t3.borrow_mut().add_child(&t2);
    t2.borrow_mut().add_child(&t5);
    t5.borrow_mut().add_child(&t1);
    t1.borrow_mut().add_child(&t3);
    assert_eq!(
        t4.borrow().get_sequence().err().unwrap(),
        "cycle [4, 3, 2, 5, 1]"
    );
}

#[test]
fn test_get_sequence_cycle_02() {
    let t1 = Task::new(1);
    let t2 = Task::new(2);
    let t3 = Task::new(3);
    let t4 = Task::new(4);
    let t5 = Task::new(5);
    let t6 = Task::new(6);
    t4.borrow_mut().add_child(&t3);
    t3.borrow_mut().add_child(&t1);
    t3.borrow_mut().add_child(&t2);
    t2.borrow_mut().add_child(&t5);
    t5.borrow_mut().add_child(&t6);
    t6.borrow_mut().add_child(&t4);
    assert_eq!(
        t4.borrow().get_sequence().err().unwrap(),
        "cycle [4, 3, 2, 5, 6]"
    );
}
