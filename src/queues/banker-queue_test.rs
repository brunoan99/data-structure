use super::*;

mod setup {
  use super::*;

  pub type BankerQueueT = BankerQueue<i32>;

  pub fn node<T>(head: T, tail: Stack<T>) -> Stack<T> {
    Stack::Node(head, Box::new(tail))
  }

  pub fn queue_empty_on_both() -> BankerQueueT {
    BankerQueueT {
      head: Stack::<i32>::Empty,
      len_head: 0,
      tail: Stack::<i32>::Empty,
      len_tail: 0,
    }
  }

  pub fn queue_filled_on_tail() -> BankerQueueT {
    BankerQueueT {
      head: Stack::<i32>::Empty,
      len_head: 0,
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
      len_tail: 4,
    }
  }

  pub fn queue_filled_on_both() -> BankerQueueT {
    BankerQueueT {
      head: node(7, node(6, node(5, node(4, Stack::Empty)))),
      len_head: 4,
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
      len_tail: 4,
    }
  }
}

#[cfg(test)]
mod new {
  use super::*;

  #[test]
  fn single_case() {
    let op = BankerQueue::<i32>::new();
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod queue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let op = BankerQueue::queue(&Stack::Empty, 0, &Stack::Empty, 0);
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_head() {
    let op = BankerQueue::queue(
      &setup::node(
        3,
        setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
      ),
      4,
      &Stack::Empty,
      0,
    );
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let op = BankerQueue::queue(
      &Stack::Empty,
      0,
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      4,
    );
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let op = BankerQueue::queue(
      &setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      4,
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      4,
    );
    let expected = BankerQueue {
      head: setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      len_head: 4,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod is_empty {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::is_empty(&queue);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::is_empty(&queue);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::is_empty(&queue);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod enqueue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::enqueue(&queue, 0);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(0, Stack::Empty),
      len_tail: 1,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::enqueue(&queue, 4);
    let expected = BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: queue.tail,
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_unballanced() {
    let queue = BankerQueue {
      head: setup::node(5, setup::node(4, Stack::Empty)),
      len_head: 2,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    let op = BankerQueue::enqueue(&queue, 6);
    let expected = BankerQueue {
      head: setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      len_head: 3,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_balanced() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::enqueue(&queue, 8);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        0,
        setup::node(
          1,
          setup::node(
            2,
            setup::node(
              3,
              setup::node(
                4,
                setup::node(
                  5,
                  setup::node(6, setup::node(7, setup::node(8, Stack::Empty))),
                ),
              ),
            ),
          ),
        ),
      ),
      len_tail: 9,
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod dequeue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::dequeue(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::dequeue(&queue);
    let expected = Some((
      0,
      BankerQueue {
        head: Stack::Empty,
        len_head: 0,
        tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
        len_tail: 3,
      },
    ));
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_unballanced() {
    let queue = BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    let op = BankerQueue::dequeue(&queue);
    let expected = Some((
      0,
      BankerQueue {
        head: setup::node(4, Stack::Empty),
        len_head: 1,
        tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
        len_tail: 3,
      },
    ));
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_balanced() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::dequeue(&queue);
    let expected = Some((
      0,
      BankerQueue {
        head: Stack::Empty,
        len_head: 0,
        tail: setup::node(
          1,
          setup::node(
            2,
            setup::node(
              3,
              setup::node(
                4,
                setup::node(5, setup::node(6, setup::node(7, Stack::Empty))),
              ),
            ),
          ),
        ),
        len_tail: 7,
      },
    ));
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod drop {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::drop(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::drop(&queue);
    let expected = Some(BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      len_tail: 3,
    });
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_unballanced() {
    let queue = BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    let op = BankerQueue::drop(&queue);
    let expected = Some(BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      len_tail: 3,
    });
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_balanced() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::drop(&queue);
    let expected = Some(BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        1,
        setup::node(
          2,
          setup::node(
            3,
            setup::node(
              4,
              setup::node(5, setup::node(6, setup::node(7, Stack::Empty))),
            ),
          ),
        ),
      ),
      len_tail: 7,
    });
    assert_eq!(op, expected)
  }
}