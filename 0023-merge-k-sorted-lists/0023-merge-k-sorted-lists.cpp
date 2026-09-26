/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
#include <vector>

class Solution {
private:
    // Helper function to merge two sorted linked lists
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        ListNode dummy(0);
        ListNode* tail = &dummy;
        
        while (l1 != nullptr && l2 != nullptr) {
            if (l1->val <= l2->val) {
                tail->next = l1;
                l1 = l1->next;
            } else {
                tail->next = l2;
                l2 = l2->next;
            }
            tail = tail->next;
        }
        
        tail->next = (l1 != nullptr) ? l1 : l2;
        return dummy.next;
    }

public:
    ListNode* mergeKLists(std::vector<ListNode*>& lists) {
        if (lists.empty()) return nullptr;
        
        int k = lists.size();
        while (k > 1) {
            int idx = 0;
            for (int i = 0; i < k; i += 2) {
                if (i + 1 < k) {
                    lists[idx++] = mergeTwoLists(lists[i], lists[i + 1]);
                } else {
                    lists[idx++] = lists[i];
                }
            }
            k = idx; // Reduce total list count to half
        }
        
        return lists[0];
    }
};