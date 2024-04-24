import { describe, test, expect } from 'bun:test';
import { DoubleLinkQueue } from '../doubleLinkQueue';

describe('DoubleLinkQueue', () => {
  test('push_first', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_first(1);
    queue.push_first(2);
    queue.push_first(3);

    expect(queue.size).toBe(3);
    expect(queue.peek_first()).toBe(3);
    expect(queue.peek_last()).toBe(1);
  })

  test('push_last', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.size).toBe(3);
    expect(queue.peek_first()).toBe(1);
    expect(queue.peek_last()).toBe(3);
  })

  test('pop_first', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.pop_first()).toBe(1);
    expect(queue.pop_first()).toBe(2);
    expect(queue.pop_first()).toBe(3);
    expect(queue.pop_first()).toBe(null);
  })

  test('pop_last', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.pop_last()).toBe(3);
    expect(queue.pop_last()).toBe(2);
    expect(queue.pop_last()).toBe(1);
    expect(queue.pop_last()).toBe(null);
  })


  test('clear', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.size).toBe(3);

    queue.clear();

    expect(queue.size).toBe(0);
  })

  test('isEmpty', () => {

    const queue = new DoubleLinkQueue<number>();

    expect(queue.isEmpty()).toBe(true);

    queue.push_last(1);

    expect(queue.isEmpty()).toBe(false);
  })


  test('peek_first', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.peek_first()).toBe(1);
    expect(queue.peek_last()).toBe(3);
  })

  test('peek_last', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.peek_last()).toBe(3);
    expect(queue.peek_first()).toBe(1);
  }) 


  test('size', () => {

    const queue = new DoubleLinkQueue<number>();

    expect(queue.size).toBe(0);

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.size).toBe(3);
  })


  test('toVec', () => {

    const queue = new DoubleLinkQueue<number>();

    queue.push_last(1);
    queue.push_last(2);
    queue.push_last(3);

    expect(queue.toVec()).toEqual([1, 2, 3]);
  })

})
