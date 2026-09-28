class Solution {
public:
    int maxDepth(string s) {
        int max_scopes = 0;
        int scopes = 0;
        for (int i = 0; i< s.size(); i++) {
            if ('(' == s[i]) { scopes++; }
            if (')' == s[i]) { scopes--; }
            max_scopes = max(max_scopes, scopes);
        }
        return max_scopes;
    }
};