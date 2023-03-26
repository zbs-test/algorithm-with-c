package JavaImpl.test;

import JavaImpl.src.LinkedStack;

public class LinkedStackTest {
    public static void main(String[] args) {
        LinkedStack<String> stack = new LinkedStack<>();
        stack.push("AAA");
        stack.push("BBB");
        stack.push("CCC");
        stack.push("DDD");
        System.out.println(stack.pop());
    }

}
