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
    bool isSymmetric(TreeNode* root) {
        // Handle all easy cases
        if (nullptr == root) { return true; }
        if ((nullptr == root->left) && (nullptr == root->right)) { return true; }
        else if (nullptr == root->left) { return false; }
        else if (nullptr == root->right) { return false; }
        // The hard case left is left and right both are not null
        // Swap left_left with right_left
        TreeNode* tmp = root->left->left;
        root->left->left = root->right->left;
        root->right->left = tmp;
        bool mirror_left = this->isSymmetric(root->left);
        bool mirror_right = this->isSymmetric(root->right);
        bool mirror_val = (root->left->val == root->right->val);
        return (mirror_left && mirror_right && mirror_val);
    }
};