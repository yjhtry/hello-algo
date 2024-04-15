
export class LinkNode<T> {
  data: T;
  next: LinkNode<T> | null;

  constructor(data: T) {
    this.data = data;
    this.next = null;
  }
}
