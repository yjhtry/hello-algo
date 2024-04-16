import { LinkNode } from "./linkNode";

export class LinkQueue<T> {
  private front: LinkNode<T> | null;
  private rear: LinkNode<T> | null;
  private _size: number;

  constructor() {
    this.front = null;
    this.rear = null;
    this._size = 0;
  }

  get size() {
    return this._size;
  }

  isEmpty() {
    return this._size === 0;
  }

  clear() {
    this.front = null;
    this.rear = null;
    this._size = 0;
  }

  push(value: T) {
    const node = new LinkNode(value);

    if (this.size === 0) {
      this.front = node;
    }

    if (this.rear) {
      this.rear.next = node;
    }

    this.rear = node;

    this._size++;
  }

  pop(): T | null {
    if (this._size === 0) {
      return null
    }

    let front = this.front;

    this.front  = front!.next

    this._size--;
    return front!.data
  }

  peek() {
    if (this._size === 0) {
      return null
    }

    return this.front!.data
  }

  toArray(): T[] {
    const result: T[] = [];

    let current = this.front;

    while (current) {
      result.push(current.data);

      current = current.next
    }

    return result;
  }
}
