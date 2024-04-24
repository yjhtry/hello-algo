export class DoubleLinkNode<T> {
  next: DoubleLinkNode<T> | null = null;
  prev: DoubleLinkNode<T> | null = null;
  data: T;

  constructor(data: T) {
    this.data = data;
  }
}
