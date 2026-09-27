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
    bool hasPathSum(TreeNode* root, int targetSum) {
        if (nullptr == root) { return false; }
        if ((targetSum == root->val) && (nullptr == root->left) && (nullptr == root->right)) { return true; }
        if (this->hasPathSum(root->left, targetSum - root->val)) { return true; }
        if (this->hasPathSum(root->right, targetSum - root->val)) { return true; }
        return false;
    }
};