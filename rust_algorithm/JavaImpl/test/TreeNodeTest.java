package JavaImpl.test;

import JavaImpl.src.TreeNode;

public class TreeNodeTest {
    public static void main(String[] args) {
        TreeNode<Character>  a = new TreeNode<>('A');
        TreeNode<Character>  b = new TreeNode<>('B');
        TreeNode<Character>  c = new TreeNode<>('C');
        TreeNode<Character>  d = new TreeNode<>('D');
        TreeNode<Character>  e = new TreeNode<>('E');
        TreeNode<Character>  f = new TreeNode<>('F');
        // ½á³ÉÊ÷
        a.left = b;
        a.right = c;
        b.left = d;
        b.right = e;
        d.left = f;

        System.out.println(a.left.left.left.element); // F
        TreeNode.preOrder(a);
        System.out.println();
        TreeNode.inOrder(a);
        System.out.println();
        TreeNode.postOrder(a);
        System.out.println();
        TreeNode.levelOrder(a);
    }
}
