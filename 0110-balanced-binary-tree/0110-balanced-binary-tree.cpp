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
    bool isBalanced(TreeNode* root) {
        if (nullptr == root) { return true; }
        size_t height_left = this->height(root->left);
        size_t height_right = this->height(root->right);
        bool balance_root = (std::abs(static_cast<int>(height_left) - static_cast<int>(height_right)) <= 1);
        bool balance_left = this->isBalanced(root->left);
        bool balance_right = this->isBalanced(root->right);
        return (balance_root && balance_left && balance_right);
    }
private:
    size_t height(TreeNode* root) {
        if (nullptr == root) { return 0; }
        return 1 + max(this->height(root->left), this->height(root->right));
    }
};