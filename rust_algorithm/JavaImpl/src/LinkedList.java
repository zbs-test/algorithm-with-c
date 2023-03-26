package JavaImpl.src;

public class LinkedList<E> {
    private static class Node<E> {
        E element;
        Node<E> next;

        // Node methods
        public Node(E element) {
            this.element = element;
        }
    }

    // ���ÿ�������
    private final Node<E> head = new Node<>(null);
    private int size = 0;

    // LinkedList methods
    public void add(int index, E element) {
        if (index < 0 || index > size)
            throw new IndexOutOfBoundsException("����λ�ò��Ϸ�");

        Node<E> prev = head;
        for (int i = 0; i < index; i++)
            prev = prev.next;
        Node<E> node = new Node<>(element);
        node.next = prev.next;
        prev.next = node;
        size++;
    }

    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder();
        Node<E> node = head.next;
        while (node != null) {
            builder.append(node.element).append(" ");
            node = node.next;
        }
        return builder.toString();
    }

    public E remove(int index) {
        if (index < 0 || index > size)
            throw new IndexOutOfBoundsException("ɾ����λ�ò��Ϸ�");
        Node<E> prev = head;
        for (int i = 0; i < index; i++)
            prev = prev.next;
        E e = prev.next.element;
        prev.next = prev.next.next;
        size--;
        return e;
    }

    public E get(int index) {
        if (index < 0 || index > size)
            throw new IndexOutOfBoundsException("��ȡ��λ�ò��Ϸ�");
        Node<E> node = head;
        while (index-- >= 0)//������index ����-1��ֹͣ
            node = node.next;
        return node.element;
    }
}
