class Solution {
public:
    int scoreOfParentheses(string s) {
        vector<size_t> stack{};
        int score = 0;
        int scopes = 0;
        for (int i = 0; i < s.size(); i++) {
            char ch = s[i];
            if ('(' == ch) {
                stack.push_back(i);
                scopes++;
            }
            // since s is assumed to be balanced, no need to validate
            if (')' == ch) {
                if (i > 0 && '('== s[i-1] ){
                    score += pow(2, (scopes-1));
                }
                stack.pop_back();
                scopes--;
            }
        }
        return score;
    }
};