class Solution {
public:
    string countAndSay(int n) {
        string res = "1";
        for (int i = 1; i < n; i ++) {
            res = this->RLE(res);
        }
        return res;
    }
private:
    string RLE(string const &str) {
        char current_digit = '*';
        int count = 0;
        string res = "";
        for (int i = 0; i<str.size(); i++) {
            const char ch = str[i];
            if (!std::isdigit(ch)) {
                // printf("Invalid: char is not digit\n");
                break;
            }
            if (ch != current_digit) {
                if (0 != count) {
                    res += std::to_string(count);
                    res += current_digit;
                }
                current_digit = ch;
                count = 1;
            } else {
                count++;
            }
        }
        // Handle the last digit, which the loop skipped
        res += std::to_string(count);
        res += current_digit;
        return res;
    }
};