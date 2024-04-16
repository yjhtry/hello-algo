export class VecQueue<T> {
  private data: T[] = []
  private _size: number = 0

  get size() {
    return this._size;
  }

  isEmpty() {
    return this._size === 0;
  }

  push(value: T) {
    this.data.push(value)

    this._size++;
  }

  pop() {
    this._size--;

    return this.data.shift() || null
  }

  peek() {
    return this.data[0] || null
  }

  toArray() {
    return this.data.slice()
  }

}
