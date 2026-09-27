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
        // std::cout << "length: " << nums.size() << std::endl;
        // for (int x : nums) {
        //     std::cout << x << " ";
        // }
        // std::cout << std::endl;
        if (0 == nums.size()) { return nullptr; }
        if (1 == nums.size()) { return new TreeNode(nums[0], nullptr, nullptr); }
        size_t center = nums.size() / 2;
        // std::cout << "current: " << nums[center] << std::endl;
        std::vector<int> slice_left(nums.begin(), nums.begin()+(center));
        std::vector<int> slice_right(nums.begin()+center+1, nums.end());
        TreeNode* left = this->sortedArrayToBST(slice_left);
        TreeNode* right = this->sortedArrayToBST(slice_right);
        TreeNode* current = new TreeNode(nums[center], left, right);
        return current;
    }
};