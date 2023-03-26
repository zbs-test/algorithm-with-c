package JavaImpl.test;
import JavaImpl.src.*;
public class ArrayListTest {
    public static void main(String[] args) {
        ArrayList<Integer> list = new ArrayList<>();
        list.add(0,1);
        list.add(1,1);
        list.add(2,1);
        System.out.println(list);
        for(int i = 3; i< 20;i++)list.add(i, 2);
        System.out.println(list);
        int i = list.remove(3);
        System.out.println(i);
        int j = list.get(1);
        System.out.println(j);
        System.out.println(list.size());
    }
}
