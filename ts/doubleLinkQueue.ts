import { DoubleLinkNode } from "./doubleLinkNode";

export class DoubleLinkQueue<T> {
  private front: DoubleLinkNode<T> | null = null;
  private rear: DoubleLinkNode<T> | null = null;
  private _size = 0

  get size() {
    return this._size;
  }

  isEmpty() {
    return this._size === 0;
  }

  push_first(value: T) {
    const node = new DoubleLinkNode(value);
    node.next = this.front;

    if (this.front !== null) {
      this.front.prev = node;
    }

    this.front = node;

    if (this.rear === null) {
      this.rear = node;
    }

    this._size++;
  }

  push_last(value: T) {
    const node = new DoubleLinkNode(value);
    node.prev = this.rear;

    if (this.rear === null) {
      this.front = node;
    } else {
      this.rear.next = node;
    }

    this.rear = node;

    this._size++;
  }

  peek_first() {
    if (this.front === null) {
      return null;
    }

    return this.front.data;
  }

  peek_last() {
    if (this.rear === null) {
      return null;
    }

    return this.rear.data;
  }

  pop_first() {
    if (this.front === null) {
      return null;
    }

    const data = this.front.data;
    this.front = this.front.next;

    if (this.front === null) {
      this.rear = null;
    }

    this._size--;

    return data;
  }

  pop_last() {
    if (this.rear === null) {
      return null
    }

    let data = this.rear.data;

    this.rear = this.rear.prev;

    this._size--;

    if (this.rear === null) {
      this.front = null;
    }

    return data;
  }

  clear() {
    this.front = null;
    this.rear = null;
    this._size = 0;
  }

  toVec() {
    let vec = [];
    let node = this.front;
    while (node !== null) {
      vec.push(node.data);
      node = node.next;
    }
    return vec;
  }

}
