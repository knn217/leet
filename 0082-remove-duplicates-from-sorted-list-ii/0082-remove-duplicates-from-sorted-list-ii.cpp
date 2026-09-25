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
class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        // Edge case
        if (!head) { return head;}
        ListNode begin = ListNode{head->val - 1};
        begin.next = head;
        ListNode* current = &begin;
        ListNode* prev = &begin;
        while (true) {
            // printf("current [%d]\n", current->val);
            if (!current) { break; }
            int val = current->val;
            ListNode* next_1 = current->next;
            if (!next_1) { break; }
            int val_next_1 = next_1->val;
            ListNode* next_2 = next_1->next;
            if (!next_2) { break; }
            int val_next_2 = next_2->val;
            if (val_next_1 == val_next_2) {
                // Don't progress, handle inside next1 and next2
                while (val_next_1 == val_next_2) {
                    // printf("next_1 [%d]\n", val_next_1);
                    // printf("next_2 [%d]\n", val_next_2);
                    next_1 = next_2;
                    if (!next_2) { break; }
                    next_2 = next_2->next;
                    if (!next_2) { break; }
                    val_next_2 = next_2->val;
                }
                // after the loop, next_2 should be different from next_1
                current->next = next_2;
            } else {
                current = current->next;
            }
        }
        return begin.next;
    }
};