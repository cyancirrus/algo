use prepatory::stack::List;

#[test]
fn stack() {
    let list = List::new();
    let list = list.prepend(1);
    assert_eq!(list.head(), Some(&1));
    assert_eq!(list.tail().head(), None);
}

#[test]
fn basics() {
    let list = List::new();
    assert_eq!(list.head(), None);

    let list = list.prepend(1).prepend(2).prepend(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);

}
