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
    ListNode* reverseKGroup(ListNode* head, int k) {
        if (!head || k == 1) {
            return head;
        }

        ListNode dummy(0, head);
        ListNode* groupPrev = &dummy;

        while (true) {
            // Check if there are at least k nodes left to reverse
            ListNode* kth = getKthNode(groupPrev, k);
            if (!kth) { break; }

            ListNode* groupNext = kth->next;
            
            // Pointer setup for reversing the current k-group segment
            ListNode* prev = groupNext;
            ListNode* curr = groupPrev->next;

            // Reverse current k-group
            while (curr != groupNext) {
                ListNode* tmp = curr->next;
                curr->next = prev;
                prev = curr;
                curr = tmp;
            }

            // Re-link the previous segment to the new head of this reversed k-group
            ListNode* tmp = groupPrev->next; // groupPrev->next is now the tail of the reversed group
            groupPrev->next = kth;
            groupPrev = tmp;
        }

        return dummy.next;
    }

private:
    // Helper function to advance k steps from current node
    ListNode* getKthNode(ListNode* curr, int k) {
        while (curr && k > 0) {
            curr = curr->next;
            --k;
        }
        return curr;
    }
};