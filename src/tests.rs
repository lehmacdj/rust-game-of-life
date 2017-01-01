use frame;

#[test]
fn frame_init() {
    let frame = frame::Frame::<i32>::new(10, 10);
    assert_eq!(frame.width(), 10);
    assert_eq!(frame.height(), 10);
    for x in 0..9 {
        for y in 0..9 {
            assert_eq!(*frame.get(x, y), i32::default());
        }
    }
}

#[test]
fn frame_mut() {
    let mut frame = frame::Frame::<i32>::new(2, 2);
    frame.set(1, 1, 1);
    assert_eq!(*frame.get(1, 1), 1)
}

#[test]
fn frame_next() {
    let mut frame1 = frame::Frame::<i32>::new(2, 2);

    let frame2 = frame1.next_frame(|board, (x, y)| {
        board.get(x, y) + 1
    });

    let val = i32::default() + 1;
    frame1.set(0, 0, val);
    frame1.set(0, 1, val);
    frame1.set(1, 0, val);
    frame1.set(1, 1, val);

    assert_eq!(frame1, frame2);
}
