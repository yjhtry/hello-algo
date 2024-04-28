import { KvPair } from "./kvPair";

export class LinearHashMap<T> {
  data: Array<KvPair<T> | null> = Array.from({length: 10}, () => null)
  capacity = 10
  capacityRatio = 2
  _size = 0
  _loadFactor = 0.75

  get size() {
    return this._size
  }

  is_empty() {
    return this._size === 0
  }

  hash_key(key: number) {
    return key % this.capacity
  }

  loadFactor() {
    return this.size / this.capacity
  }

  findBucket(key: number) {
    let index = this.hash_key(key)
    let kv = this.data[index]
    let tombstoneFrom = -1
    
    while (kv !== null) {
      if (kv.key === key) {
        // 交换已删除的位置, 提高查找效率
        if (tombstoneFrom !== -1) {
          this.data[tombstoneFrom] = kv
          this.data[index] = new KvPair(-1, null as T)

          return tombstoneFrom
        }

        return index
      }

      if (kv.key === -1 && tombstoneFrom === -1) {
        tombstoneFrom = index
      }

      index = (index + 1) % this.capacity
    }

    return tombstoneFrom === -1 ? index : tombstoneFrom
  }

  insert(key: number, value: T) {
    if (this.loadFactor() > this._loadFactor) {
      this.extend()
    }

    let index = this.findBucket(key)

    this.data[index] = new KvPair(key, value)
    this._size++
  }

  remove(key: number) {
    if (this._size === 0) {
      return
    }

    let index = this.findBucket(key)

    if (this.data[index] !== null) {
      this.data[index] = new KvPair(-1, null as T)
      this._size--
    }
  }

  get(key: number) {
    if (this._size === 0) {
      return null
    }

    let index = this.findBucket(key)

    return this.data[index]?.value || null;
  }

  extend() {
    let oldData = this.data

    this.capacity *= this.capacityRatio
    this.data = Array.from({length: this.capacity}, () => null)
    this._size = 0

    for (let kv of oldData) {
      if (kv !== null && kv.key !== -1) {
        this.insert(kv.key, kv.value)
      }
    }
  }
}
