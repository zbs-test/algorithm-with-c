package JavaImpl.src;

import java.util.NoSuchElementException;

public class LinkedStack<E> {
    private static class Node<E> {
        E element;
        Node<E> next;

        public Node(E element) {
            this.element = element;
        }
    }

    private final Node<E> head = new Node<>(null);

    // LinkedStack methods
    public void push(E element) {
        Node<E> node = new Node<>(element);
        node.next = head.next;
        head.next = node;
    }

    public E pop() {
        if (head.next == null)
            throw new NoSuchElementException("ø’’ª");
        E e = head.next.element;
        head.next = head.next.next;
        return e;
    }
}
