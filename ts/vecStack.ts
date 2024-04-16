export class VecStack<T> {
  private data: T[] = []
  private _size: number = 0

  get size() {
    return this._size
  }

  isEmpty() {
    return this._size === 0
  }

  push(data: T) {
    this.data.unshift(data)

    this._size++;
  }

  pop(): T | null {
    this._size--;

    return this.data.shift() || null
  }

  peek() {
    return this.data[0] || null
  }

  toArray() {
  return  this.data.slice()
  }
}
