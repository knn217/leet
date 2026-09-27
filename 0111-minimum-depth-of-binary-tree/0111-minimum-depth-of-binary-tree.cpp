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
    int minDepth(TreeNode* root) {
        if (nullptr == root) { return 0; }
        int depth_left = this->minDepth(root->left);
        if (1 == depth_left) { return (1 + depth_left); }
        int depth_right = this->minDepth(root->right);
        if (1 == depth_right) { return (1 + depth_right); }
        if (0 == depth_left) { return (1 + depth_right); }
        if (0 == depth_right) { return (1 + depth_left); }
        return 1 + min(depth_left, depth_right);
    }
};