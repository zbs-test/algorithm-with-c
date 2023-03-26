package JavaImpl.src;

import java.util.NoSuchElementException;

public class LinkedQueue<E> {
    private static class Node<E> {
        E element;
        Node<E> next;

        public Node(E element) {
            this.element = element;
        }
    }

    private final Node<E> head = new Node<>(null);

    // methods
    public void offer(E element) {
        Node<E> last = head;
        while (last.next != null)
            last = last.next;
        last.next = new Node<>(element);
    }

    public E poll() {
        if (head.next == null)
            throw new NoSuchElementException("¿Õ¶Ó");
        E e = head.next.element;
        head.next = head.next.next;
        return e;
    }

    public boolean isEmpty(){
        return head.next == null;
    }
}
