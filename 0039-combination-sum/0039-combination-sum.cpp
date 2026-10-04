#include <vector>
#include <algorithm>

class Solution {
public:
    std::vector<std::vector<int>> combinationSum(std::vector<int>& candidates, int target) {
        std::vector<std::vector<int>> results;
        std::vector<int> current;
        
        // Sorting allows early pruning when candidate > target
        std::sort(candidates.begin(), candidates.end());
        
        this->backtrack(candidates, target, 0, current, results);
        return results;
    }

private:
    void backtrack(const std::vector<int>& candidates, int remaining, int start_index,
                   std::vector<int>& current, std::vector<std::vector<int>>& results) {
        if (remaining == 0) {
            results.push_back(current);
            return;
        }

        for (size_t i = start_index; i < candidates.size(); ++i) {
            // Pruning: Since candidates are sorted, if the current element exceeds 
            // the remaining target, all subsequent candidates will too.
            if (candidates[i] > remaining) {
                break;
            }

            // Choose
            current.push_back(candidates[i]);
            
            // Explore (stay at index `i` since elements can be reused)
            this->backtrack(candidates, remaining - candidates[i], i, current, results);
            
            // Unchoose (backtrack)
            current.pop_back();
        }
    }
};