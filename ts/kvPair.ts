export class KvPair<T> {
  key: number
  value: T

  constructor(key: number, value: T) {
    this.key = key
    this.value = value
  }
}
