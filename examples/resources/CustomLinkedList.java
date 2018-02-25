import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * Just a very simple linked list implementation
 */
public class CustomLinkedList<E> implements Iterable<E> {
    private Element<E> start;
    private int size;

    public static void main(String[] args) {
        CustomLinkedList<String> list = new CustomLinkedList<>();
        list.addFirst("!!!");
        list.removeFirst();
        list.addFirst("world");
        list.addFirst("hello");

        for (String s : list) {
            System.out.println(s);
        }

        System.out.println("size = " + list.size());
    }

    public void addFirst(E value) {
        start = new Element<>(value, start);
        size++;
    }

    public void removeFirst() {
        if (start != null) {
            start = start.next;
            size--;
        }
    }

    public int size() {
        return size;
    }

    @Override
    public Iterator<E> iterator() {
        return new Iter<>(start);
    }

    private class Element<E> {
        E value;
        Element<E> next;

        Element(E value, Element<E> next) {
            this.value = value;
            this.next = next;
        }
    }

    private class Iter<E> implements Iterator<E> {
        Element<E> next;

        Iter(Element<E> start) {
            next = start;
        }

        @Override
        public boolean hasNext() {
            return next != null;
        }

        @Override
        public E next() {
            if (next == null) {
                throw new NoSuchElementException();
            }

            Element<E> prev = next;
            next = prev.next;
            return prev.value;
        }
    }
}
