use std::mem;

pub struct List {
    head: Link,
}


#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

impl List{
    pub fn new() -> Self {
        List{ head:Link::Empty}
    }
    
    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node{
            elem:elem,
            // 从借用 self 中偷出了它的值 head 并赋予给 next 字段
            // 同时将一个新值 Link::Empty 放入到 head 中，成功完成偷梁换柱
            next:std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        //我们将 self.head 的值偷出来，然后再将 Link::Empty 填回到 self.head 中
        // 此时用于 match 匹配的就是一个拥有所有权的值类型，而不是之前的引用类型。
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            // 为什么这里是mut boxed_node而不是引用了？ 因为我们需要他的所有权，这样才能释放掉
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
        }
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        use super::List;
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(),Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(),Some(5));
        assert_eq!(list.pop(),Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}