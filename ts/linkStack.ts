import {LinkNode  } from './linkNode';


export class LinkStack<T> {
  private head: LinkNode<T> | null;
  private _size: number;

  constructor() {
    this.head = null;
    this._size = 0;
  }

  get size() {
    return this._size;
  }

  push(data: T) {
    const node = new LinkNode(data);
    node.next = this.head;

    this.head = node;

    this._size += 1;
  }

  pop() {
    if (this.head === null) {
      return null;
    }

    let head = this.head;

    this.head = head.next;

    head.next = null;
    this._size -= 1;

    return head
  }

  peek() {
    return this.head;
  }

  isEmpty() {
    return this.head === null;
  }

  toArray() {
    const res = [] as T[];

    let current = this.head;

    while (current !== null) {
      res.push(current.data);
      current = current.next;
    }

    return res
  }
}
