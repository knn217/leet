class Solution {
public:
    string countAndSay(int n) {
        if (n == 1) return "1";
        
        string res = "1";
        res.reserve(4500); // Reserve enough memory to avoid reallocations up to n=30
        
        for (int i = 1; i < n; ++i) {
            string next;
            next.reserve(res.size() * 2); // Reserve space for the next sequence
            
            int len = res.size();
            for (int j = 0; j < len; ) {
                int k = j;
                while (k < len && res[k] == res[j]) {
                    k++;
                }
                
                int count = k - j;
                next.push_back('0' + count); // Direct char conversion (fast)
                next.push_back(res[j]);
                
                j = k; // Jump to the next distinct digit run
            }
            res = std::move(next); // Zero-copy move assignment
        }
        
        return res;
    }
};