class Solution {
public:
    bool checkValidString(string s) {
        vector<size_t> opening{};
        vector<size_t> wildcard{};
        for (int i = 0; i < s.size(); i++) {
            char ch = s[i];
            if ('*' == ch) { wildcard.push_back(i); }
            if ('(' == ch) { opening.push_back(i); }
            if (')' == ch) {
                // If there are still opening, use them
                if (0 != opening.size()) {
                    opening.pop_back();
                    continue;
                }
                // If opening is empty but there are wildcard left, use them
                if (0 != wildcard.size()) {
                    wildcard.pop_back();
                    continue;
                }
                // If out of both, but still meet ')', invalid
                return false;
            }
        }
        while (opening.size() > 0 && wildcard.size() > 0) {
            int last_opening_idx = opening.back();
            int last_wildcard_idx = wildcard.back();
            // Only a wildcard after the opening can resolve it
            // if the last wildcard is earlier than opening, it cannot be resolved => invalid
            if (last_wildcard_idx <= last_opening_idx) { return false; }
            opening.pop_back();
            wildcard.pop_back();
        }

        // After handling, if openings left > 0 => invalid
        if (opening.size() > 0) { return false; }
        return true;
    }
};