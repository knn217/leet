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
    TreeNode* sortedArrayToBST(vector<int>& nums) {
        return this->buildBST(nums, 0, nums.size() - 1);
    }
private:
    TreeNode* buildBST(const vector<int>& nums, int left, int right) {
        if (left > right) { return nullptr; }
        // Avoid potential integer overflow compared to (left + right) / 2
        int mid = left + (right - left) / 2;
        TreeNode* node = new TreeNode(nums[mid]);
        node->left = this->buildBST(nums, left, mid - 1);
        node->right = this->buildBST(nums, mid + 1, right);
        return node;
    }
};