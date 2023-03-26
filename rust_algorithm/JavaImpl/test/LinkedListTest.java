package JavaImpl.test;
import JavaImpl.src.*;
public class LinkedListTest {
    public static void main(String[] args) {
        LinkedList<Integer> list = new LinkedList<>();
        list.add(0, 1);
        list.add(1, 1);
        list.add(2, 1);
        list.add(3, 1);
        // list.add(9, 1);
        System.out.println(list);
        int i = list.remove(2);
        System.out.println(i);
        System.out.println(list);
        int j = list.get(1);
        System.out.println(j);
    }
}
