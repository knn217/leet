/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        if (p == q) { return true; }
        else if (nullptr == p) { return false; }
        else if (nullptr == q) { return false; }
        bool same_left = this->isSameTree(p->left, q->left);
        bool same_root = (p->val == q->val);
        bool same_right = this->isSameTree(p->right, q->right);
        return (same_left && same_root && same_right);
    }
};