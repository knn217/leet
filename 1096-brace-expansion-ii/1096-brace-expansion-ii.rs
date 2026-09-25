use std::collections::HashSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        // Stack stores (groups_to_union, current_concatenation_group)
        let mut stack: Vec<(Vec<HashSet<String>>, Vec<HashSet<String>>)> = Vec::new();
        
        let mut groups_to_union: Vec<HashSet<String>> = Vec::new();
        let mut current_group: Vec<HashSet<String>> = vec![HashSet::from([String::new()])];

        for c in expression.chars() {
            match c {
                '{' => {
                    // Push current context to stack and reset local scope
                    stack.push((groups_to_union, current_group));
                    groups_to_union = Vec::new();
                    current_group = vec![HashSet::from([String::new()])];
                }
                'a'..='z' => {
                    // Append character to every string in the active concatenation set
                    if let Some(last_set) = current_group.last_mut() {
                        let mut new_set = HashSet::with_capacity(last_set.len());
                        for s in last_set.iter() {
                            let mut new_str = String::with_capacity(s.len() + 1);
                            new_str.push_str(s);
                            new_str.push(c);
                            new_set.insert(new_str);
                        }
                        *last_set = new_set;
                    }
                }
                ',' => {
                    // Comma ends current product group and adds it to the union group
                    groups_to_union.append(&mut current_group);
                    current_group = vec![HashSet::from([String::new()])];
                }
                '}' => {
                    // Complete union for the scope inside { ... }
                    groups_to_union.append(&mut current_group);
                    let mut evaluated_set = HashSet::new();
                    for set in groups_to_union.drain(..) {
                        evaluated_set.extend(set);
                    }

                    // Restore outer scope context
                    let (prev_union, mut prev_group) = stack.pop().unwrap();
                    groups_to_union = prev_union;

                    // Compute Cartesian product: prev_group[-1] × evaluated_set
                    if let Some(last_set) = prev_group.last_mut() {
                        let mut new_set = HashSet::with_capacity(last_set.len() * evaluated_set.len());
                        for prefix in last_set.iter() {
                            for suffix in &evaluated_set {
                                let mut combined = String::with_capacity(prefix.len() + suffix.len());
                                combined.push_str(prefix);
                                combined.push_str(suffix);
                                new_set.insert(combined);
                            }
                        }
                        *last_set = new_set;
                    }
                    current_group = prev_group;
                }
                _ => {}
            }
        }

        // Final union for the top-level expression
        groups_to_union.append(&mut current_group);
        let mut res_set: HashSet<String> = HashSet::new();
        for set in groups_to_union {
            res_set.extend(set);
        }

        // Sort into lexicographical vector
        let mut result: Vec<String> = res_set.into_iter().collect();
        result.sort_unstable();
        result
    }
}