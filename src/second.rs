type NodePtr<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    head: NodePtr<T>,
}
struct Node<T> {
    elem: T,
    next: NodePtr<T>,
}

pub struct IntoIter<T>(NodePtr<T>);

#[warn(dead_code)]
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Option::None }
    }
    pub fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
            elem: val,
            next: self.head.take(),
        });
        self.head = Option::Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        // 移动借用结构体成员的所有权
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // let mut next = self.head.take();

        // let elem = next.unwrap();
        // self.head = elem.next;
        // return Some(elem.elem);
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take()
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
