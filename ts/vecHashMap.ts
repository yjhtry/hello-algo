import { KvPair } from "./kvPair";

export class VecHashMap<T> {
  data: Array<KvPair<T>[]> = Array.from({length: 10}, () => [])
  capacity = 10
  extendRatio = 2
  _loadFactor = 0.75
  _size = 0

  get size() {
    return this._size
  }

  is_empty() {
    return this._size === 0;
  }

  hash_key(key: number) {
    return key % this.capacity
  }

  loadFactor() {
    return this._size / this.capacity
  }

  insert(key: number, value: T) {
    if (this.loadFactor() > this._loadFactor) {
      this.extend()
    }

    const index = this.hash_key(key)
    const kv = new KvPair(key, value)
    const bucket = this.data[index]

    bucket.push(kv)

    this._size++
  }

  get(key: number) {
    if (this._size === 0) {
      return null
    }

    const index = this.hash_key(key);
    const bucket = this.data[index]

    for (const kv of bucket) {
      if (kv.key === key) {
        return kv.value
      }
    }
  }

  remove(key: number) {
    if (this._size === 0) {
      return null
    }

    const bucket = this.data[this.hash_key(key)]

    for (let i = 0; i < bucket.length; i++) {
      let kv = bucket[i]

      if (kv.key === key) {
        bucket.splice(i, 1)
        this._size--
        return kv.value
      }
    }

    return null
  }

  extend() {
    const oldData = this.data
    this.capacity *= this.extendRatio
    this.data = Array.from({length: this.capacity}, () => [])
    this._size = 0

    for (const bucket of oldData) {
      for (const kv of bucket) {
        this.insert(kv.key, kv.value)
      }
    }
  }
}
