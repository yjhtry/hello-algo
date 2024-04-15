import { expect, test, describe } from "bun:test";
import { LinkStack  } from '../linkStack';

describe("arithmetic", () => {
  
  test("stack isEmpty should work", () => {
    const stack = new LinkStack<number>();
    expect(stack.isEmpty()).toBeTrue()
  });

  test("stack push should work", () => {
    const stack = new LinkStack<number>();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    expect(stack.size).toBe(3)
  });

  test("stack pop should work", () => {
    const stack = new LinkStack<number>();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    console.log(stack.toArray())
    expect(stack.pop()!.data).toBe(3)

    expect(stack.size).toBe(2)
  })

  test("stack peek should work", () => {
    const stack = new LinkStack<number>();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    expect(stack.peek()!.data).toBe(3)
  })

  test("stack toArray should work", () => {
    const stack = new LinkStack<number>();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    expect(stack.toArray()).toEqual([3, 2, 1])
  })

});
