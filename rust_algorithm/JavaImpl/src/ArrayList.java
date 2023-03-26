package JavaImpl.src;

public class ArrayList<E> {
    int capacity = 10;
    int size = 0;
    private Object[] array = new Object[capacity];

    // methods
    public void add(int index, E element) {
        if (index < 0 || index > size)
            throw new IndexOutOfBoundsException("插入的index不合法,合法范围为:0-" + size);
        // 扩容
        if (capacity == size) {
            int newCapacity = capacity + (capacity >> 1);
            Object[] newArray = new Object[newCapacity];
            System.arraycopy(array, 0, newArray, 0, size);
            array = newArray;
            capacity = newCapacity;
        }
        // 搬运再插入
        for (int i = size; i > index + 1; i--)
            array[i] = array[i - 1];
        array[index] = element;
        size++;

    }

    public String toString() {
        StringBuilder builder = new StringBuilder();
        for (int i = 0; i < size; i++)
            builder.append(array[i]).append(" ");
        return builder.toString();
    }

    @SuppressWarnings("unchecked")
    public E remove(int index) {
        if (index < 0 || index > size)
            throw new IndexOutOfBoundsException("删除的位置不合法");
        E e = (E) array[index];
        for (int i = index; i < size; i++)
            array[i] = array[i + 1];
        size--;
        return e;
    }

    @SuppressWarnings("unchecked")
    public E get(int index){
        if(index < 0||index>size-1)throw new IndexOutOfBoundsException("获取的位置不合法");
        return (E) array[index];
    }

    public int size(){
        return size;
    }

}