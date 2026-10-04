class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        span<int> span_full{candidates};
        return this->recurs(span_full, target);
    }
private:
    vector<vector<int>> recurs(span<int>& candidates, int target) {
        // printf("Getting combinations for target (%d)\n", target);
        vector<vector<int>> res{};
        // if (0 > target) { return res;}
        if (0 == target) {
            res.push_back(vector<int>());
            return res;
        }
        for (int i = 0; i < candidates.size(); i++) {
            int candidate = candidates[i];
            int new_target = target - candidate;
            if (0 > new_target) { continue; }
            auto new_candidates = candidates.subspan(i);
            auto sub_vecs = this->recurs(new_candidates, new_target);
            // if (0 == sub_vecs.size()) { continue; }
            for (auto &vec : sub_vecs) {
                vec.push_back(candidate);
            }
            res.insert(res.end(), sub_vecs.begin(), sub_vecs.end());
        }
        return res;
    }
};