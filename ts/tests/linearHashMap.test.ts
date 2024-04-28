import { describe, expect, test  } from 'bun:test';

import { LinearHashMap } from '../linearHashMap';

describe('LinearHashMap test', () => {
  test('insert', () => {
    const map = new LinearHashMap<number>()
    map.insert(1, 1)
    expect(map.size).toBe(1)
    map.insert(2, 2)
    expect(map.size).toBe(2)
    map.insert(3, 3)
    expect(map.size).toBe(3)

  })

  test('get', () => {
    const map = new LinearHashMap<number>()
    map.insert(1, 1)
    map.insert(2, 2)
    map.insert(3, 3)
    expect(map.get(1)).toBe(1)
    expect(map.get(2)).toBe(2)
    expect(map.get(3)).toBe(3)
  })

  test('remove', () => {
    const map = new LinearHashMap<number>()
    map.insert(1, 1)
    map.insert(2, 2)
    map.insert(3, 3)
    map.remove(1)
    expect(map.size).toBe(2)
    map.remove(2)
    expect(map.size).toBe(1)
    map.remove(3)
    expect(map.size).toBe(0)
  })


  test('is_empty', () => {
    const map = new LinearHashMap<number>()
    expect(map.is_empty()).toBe(true)
    map.insert(1, 1)
    expect(map.is_empty()).toBe(false)
    map.remove(1)
    expect(map.is_empty()).toBe(true)
  })

})
