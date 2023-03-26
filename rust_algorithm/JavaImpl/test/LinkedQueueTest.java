package JavaImpl.test;

import JavaImpl.src.LinkedQueue;

public class LinkedQueueTest {
    public static void main(String[] args) {
        LinkedQueue<String> queue = new LinkedQueue<>();
        queue.offer("AAA");
        queue.offer("BBB");
        System.out.println(queue.poll());
    }
}
