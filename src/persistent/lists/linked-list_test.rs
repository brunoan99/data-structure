use super::*;

mod setup {
  use super::*;

  pub type LinkedListT = LinkedList<i32>;

  pub fn node<T>(value: T, next: Link<T>) -> Link<T> {
    Some(Box::new(ListNode { value, next: next }))
  }

  pub fn linked_list_empty() -> LinkedListT {
    LinkedListT { root: None }
  }

  pub fn linked_list_filled() -> LinkedListT {
    LinkedListT {
      root: node(0, node(1, node(2, node(3, None)))),
    }
  }
}

#[cfg(test)]
mod new {
  use super::*;

  #[test]
  fn single_case() {
    let op = LinkedList::<i32>::new();
    let expected = setup::linked_list_empty();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod is_empty {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::is_empty(&list);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::is_empty(&list);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod insert_at_beginning {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::insert_at_beginning(&list, 0);
    let expected = LinkedList {
      root: setup::node(0, list.root),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::insert_at_beginning(&list, -1);
    let expected = LinkedList {
      root: setup::node(-1, list.root),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod insert_at_end {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::insert_at_end(&list, 0);
    let expected = LinkedList {
      root: setup::node(0, list.root),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::insert_at_end(&list, 4);
    let expected = LinkedList {
      root: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, setup::node(4, None)))),
      ),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod insert_before {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::insert_before(&list, 0, &4);
    let expected = Err(InsertError::BeforeItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_without_found_item() {
    let list = setup::linked_list_filled();
    let op = LinkedList::insert_before(&list, 6, &5);
    let expected = Err(InsertError::BeforeItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_found_item() {
    let list = setup::linked_list_filled();
    let op = LinkedList::insert_before(&list, 4, &2);
    let expected = Ok(LinkedList {
      root: setup::node(
        0,
        setup::node(1, setup::node(4, setup::node(2, setup::node(3, None)))),
      ),
    });
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod insert_after {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::insert_after(&list, 0, &1);
    let expected = Err(InsertError::AfterItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_without_found_item() {
    let list = setup::linked_list_filled();
    let op = LinkedList::insert_after(&list, 6, &5);
    let expected = Err(InsertError::AfterItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_found_item() {
    let list = setup::linked_list_filled();
    let op = LinkedList::insert_after(&list, 4, &2);
    let expected = Ok(LinkedList {
      root: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(4, setup::node(3, None)))),
      ),
    });
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod remove_first {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::remove_at_beginning(&list);
    let expected = Err(RemoveError::EmptyList);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::remove_at_beginning(&list);
    let expected = Ok(LinkedList {
      root: setup::node(1, setup::node(2, setup::node(3, None))),
    });
    assert_eq!(op, expected)
  }
}
#[cfg(test)]
mod remove_last {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::remove_at_end(&list);
    let expected = Err(RemoveError::EmptyList);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::remove_at_end(&list);
    let expected = Ok(LinkedList {
      root: setup::node(0, setup::node(1, setup::node(2, None))),
    });
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod remove_item {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::remove_item(&list, 0);
    let expected = Err(RemoveError::EmptyList);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_without_found_item() {
    let list = setup::linked_list_filled();
    let op = LinkedList::remove_item(&list, 4);
    let expected = Err(RemoveError::ItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_found_item() {
    let list = setup::linked_list_filled();
    let op = LinkedList::remove_item(&list, 2);
    let expected = Ok(LinkedList {
      root: setup::node(0, setup::node(1, setup::node(3, None))),
    });
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod len {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::len(&list);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::len(&list);
    assert_eq!(op, 4)
  }
}

#[cfg(test)]
mod rev {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::rev(&list);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::rev(&list);
    let expected = LinkedList {
      root: setup::node(3, setup::node(2, setup::node(1, setup::node(0, None)))),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod concat {
  use super::*;

  #[test]
  fn to_both_empty() {
    let l1 = setup::linked_list_empty();
    let l2 = setup::linked_list_empty();
    let op = LinkedList::concat(&l1, &l2);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_first_filled_second_empty() {
    let l1 = setup::linked_list_empty();
    let l2 = setup::linked_list_empty();
    let op = LinkedList::concat(&l1, &l2);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_first_empty_second_filled() {
    let l1 = setup::linked_list_empty();
    let l2 = setup::linked_list_empty();
    let op = LinkedList::concat(&l1, &l2);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_both_filled() {
    let l1 = setup::linked_list_empty();
    let l2 = setup::linked_list_empty();
    let op = LinkedList::concat(&l1, &l2);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod split {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::split(&list, |_| true);
    let expected = (LinkedList { root: None }, LinkedList { root: None });
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::split(&list, |item| item % 2 == 0);
    let expected = (
      LinkedList {
        root: setup::node(1, setup::node(3, None)),
      },
      LinkedList {
        root: setup::node(0, setup::node(2, None)),
      },
    );
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod any {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::any(&list, |_| true);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_only_false_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::any(&list, |item| item > &3);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_with_true_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::any(&list, |item| item > &1);
    assert_eq!(op, true)
  }
}

#[cfg(test)]
mod all {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::all(&list, |_| false);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_only_true_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::all(&list, |item| item < &4);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_with_true_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::all(&list, |item| item < &1);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod find {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::find(&list, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_only_false_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::find(&list, |item| item > &3);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_with_true_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::find(&list, |item| item > &1);
    assert_eq!(op, Some(2))
  }
}

#[cfg(test)]
mod find_r {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::find_r(&list, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_only_false_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::find_r(&list, |item| item > &3);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_with_true_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::find_r(&list, |item| item > &1);
    assert_eq!(op, Some(3))
  }
}

#[cfg(test)]
mod map {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::map(&list, |item| item + 1);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::map(&list, |item| item + 1);
    let expected = LinkedList {
      root: setup::node(1, setup::node(2, setup::node(3, setup::node(4, None)))),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod filter {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::filter(&list, |_| false);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_only_false_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::filter(&list, |item| item > &3);
    let expected = LinkedList { root: None };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_true_return() {
    let list = setup::linked_list_filled();
    let op = LinkedList::filter(&list, |item| item > &1);
    let expected = LinkedList {
      root: setup::node(2, setup::node(3, None)),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod reduce {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::reduce(&list, |_, _| 4, 0);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::reduce(&list, |item, acc| item + acc, 0);
    assert_eq!(op, 6)
  }
}
