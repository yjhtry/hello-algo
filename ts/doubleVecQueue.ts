export class DoubleVecQueue<T> {
  private _data: T[] = []
  private _size  = 0

  get size() {
    return this._size
  }

  isEmpty() {
    return this._size === 0
  }

  clear() {
    this._data = []
    this._size = 0
  }

  push_first(value: T) {
    this._data.unshift(value)
    this._size++
  }

  push_last(value: T) {
    this._data.push(value)
    this._size++
  }

  peek_first() {
    if (this._size === 0) {
      return null
    }

    return this._data[0]
  }

  peek_last() {
    if (this._size === 0) {
      return null
    }

    return this._data[this._size - 1]
  }

  pop_first() {
    if (this._size === 0) {
      return null
    }

    this._size--
    return this._data.shift()
  } 

  pop_last() {
    if (this._size === 0) {
      return null
    }

    this._size--
    return this._data.pop()
  }

  toVec() {
    return this._data.slice()
  }
}
