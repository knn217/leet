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
    vector<int> inorderTraversal(TreeNode* root) {
        if (nullptr == root) {
            vector<int> empty{};
            return empty;
        }
        vector<int> left = this->inorderTraversal(root->left);
        vector<int> right = this->inorderTraversal(root->right);
        vector<int> middle{ root->val };
        left.insert(left.end(), middle.begin(), middle.end());
        left.insert(left.end(), right.begin(), right.end());
        return left;
    }
};