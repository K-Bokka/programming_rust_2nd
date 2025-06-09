pub fn run() {
    println!("16.4 BinaryHeap");

    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::from([2, 3, 11, 6, 9, 5]);

    use std::collections::binary_heap::PeekMut;
    if let Some(top) = heap.peek_mut() {
        if *top > 10 {
            let pop = PeekMut::pop(top);
            println!("pop: {:?}", pop);
        }
    }
    println!("heep: {:?}", heap);

    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(6));
    assert_eq!(heap.pop(), Some(5));
}
