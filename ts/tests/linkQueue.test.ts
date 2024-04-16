import { describe, test, expect  } from 'bun:test';
import {LinkQueue  } from '../linkQueue';

describe("test linkQueue", () => {
  test("queue isEmpty should work", () => {

    const queue = new LinkQueue<number>();
    expect(queue.isEmpty()).toBeTrue()
  })

  test("test push should work", () => {
    const queue = new LinkQueue<number>();
    queue.push(1);
    queue.push(2);
    queue.push(3);

    expect(queue.size).toBe(3)
  })

  test("test pop should work", () => {
    const queue = new LinkQueue<number>();
    queue.push(1);
    queue.push(2);
    queue.push(3);

    expect(queue.pop()).toBe(1)

    expect(queue.size).toBe(2)
  })

  test("test peek should work", () => {
    const queue = new LinkQueue<number>();
    queue.push(1);
    queue.push(2);
    queue.push(3);

    expect(queue.peek()).toBe(1)
  })

  test("test toArray should work", () => {
    const queue = new LinkQueue<number>();
    queue.push(1);
    queue.push(2);
    queue.push(3);

    expect(queue.toArray()).toEqual([1, 2, 3])
  })
})
