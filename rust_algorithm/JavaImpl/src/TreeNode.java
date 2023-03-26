package JavaImpl.src;


public class TreeNode<E> {
    public E element;
    public TreeNode<E> left, right;

    public TreeNode(E element) {
        this.element = element;
    }

    public static <T> void preOrder(TreeNode<T> root) {
        if (root == null)
            return;
        System.out.print(root.element + " ");
        preOrder(root.left);
        preOrder(root.right);
    }

    public static <T> void inOrder(TreeNode<T> root) {
        if (root == null)
            return;
        inOrder(root.left);
        System.out.print(root.element + " ");
        inOrder(root.right);
    }

    public static <T> void postOrder(TreeNode<T> root) {
        if (root == null)
            return;
        postOrder(root.right);
        System.out.print(root.element + " ");
        postOrder(root.left);
    }

    // using LinkedQueue ≤„–Ú±È¿˙
    public static <T> void levelOrder(TreeNode<T> root) {
        LinkedQueue<TreeNode<T>> queue = new LinkedQueue<>();
        queue.offer(root);
        while (!queue.isEmpty()) {
            TreeNode<T> node = queue.poll();
            System.out.print(node.element+" ");
            if (node.left != null)
                queue.offer(node.left);
            if (node.right != null)
                queue.offer(node.right);
        }
    }
}