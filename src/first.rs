pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            elem: val,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        // 移动借用结构体成员的所有权
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {}
            Link::More(node) => {
                self.head = node.next;
                return Some(node.elem);
            }
        }
        return None;
    }
}
impl Drop for List {
    fn drop(&mut self) {
        let mut link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = link {
            link = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        // test drop
        for i in 1..100000 {
            list.push(i);
        }
    }
}
