// use std::collections::HashSet;
use std::string::String;

impl Solution {
    fn extract_obj(expression: &[u8], idx_start: usize, idx_end: usize) -> Vec<String> {
        if idx_start == idx_end { return vec![]; }
        // println!("Extracting obj from |{}|", String::from_utf8_lossy(&expression[idx_start..idx_end]).to_string());
        let mut vec: Vec<String> = vec![];
        let mut obj_start = idx_start;
        let mut obj_end = idx_end;
        let mut scope_start = idx_start;
        let mut scope_end = idx_end;
        let mut scopes = 0;
        let mut prefix: Vec<String> = vec![];
        let mut suffix: Vec<String> = vec![];
        for idx in idx_start..idx_end {
            let ch = expression[idx] as char;
            if ch == '{' {
                // println!("Begin scope {} at {}", scopes, idx);
                if scopes == 0 {
                    scope_start = idx;
                    //=========================
                    suffix = Self::extract_obj(expression, obj_start, scope_start);
                    // println!("prefix for {}: {:?}", String::from_utf8_lossy(&expression[obj_start..scope_start]).to_string(), prefix);
                    // println!("suffix for {}: {:?}", String::from_utf8_lossy(&expression[obj_start..scope_start]).to_string(), suffix);
                    if !suffix.is_empty(){
                        if prefix.is_empty() {
                            prefix = suffix;
                        } else {
                            let mut vec_combine: Vec<String> = vec![];
                            for pre in &prefix {
                                for suf in &suffix {
                                    vec_combine.push(format!("{pre}{suf}"));
                                }
                            }
                            prefix = vec_combine;
                        }
                    }
                    // println!("prefix for {}: {:?}", String::from_utf8_lossy(&expression[obj_start..scope_start]).to_string(), prefix);
                }
                scopes += 1;
            }
            if ch == '}' {
                scopes -= 1;
                // println!("End scope {} at {}", scopes, idx);
                if scopes == 0 {
                    scope_end = idx + 1;
                    obj_start = scope_end;
                    //=========================
                    suffix = Self::extract_obj(expression, scope_start+1, scope_end-1);
                    // println!("prefix for {}: {:?}", String::from_utf8_lossy(&expression[(scope_start+1)..(scope_end-1)]).to_string(), prefix);
                    // println!("suffix for {}: {:?}", String::from_utf8_lossy(&expression[(scope_start+1)..(scope_end-1)]).to_string(), suffix);
                    if !suffix.is_empty(){
                        if prefix.is_empty() {
                            prefix = suffix;
                        } else {
                            let mut vec_combine: Vec<String> = vec![];
                            for pre in &prefix {
                                for suf in &suffix {
                                    vec_combine.push(format!("{pre}{suf}"));
                                }
                            }
                            prefix = vec_combine;
                        }
                    }
                    // println!("prefix for {}: {:?}", String::from_utf8_lossy(&expression[(scope_start+1)..(scope_end-1)]).to_string(), prefix);
                }
            }
            if (ch == ',' && scopes == 0) || (idx == (idx_end-1)) {
                if idx == (idx_end-1) { obj_end = idx_end; }
                else { obj_end = idx; }
                // println!("End object: {}..{}", obj_start, obj_end);
                // End of an object, extract the obj
                let obj = String::from_utf8_lossy(&expression[obj_start..obj_end]).to_string();
                // println!("obj: {:?}", obj);
                // println!("prefix: {:?}", prefix);
                if prefix.is_empty() {
                    // println!("vec: {:?}", vec);
                    vec.push(obj.clone());
                } else {
                    let mut vec_combine: Vec<String> = vec![];
                    for pre in &prefix {
                        vec_combine.push(format!("{pre}{obj}"));
                    }
                    // Add and reset prefix
                    vec.extend(vec_combine);
                    prefix = vec![];
                    // suffix = vec![];
                }
                obj_start = idx + 1;
                continue;
            }
            obj_end = idx + 1;
            println!("obj: {:?}", String::from_utf8_lossy(&expression[obj_start..obj_end]).to_string());
        }
        // println!("End all");
        // println!("|{}| Returning objs: {:?}", String::from_utf8_lossy(&expression[idx_start..idx_end]).to_string(), vec);
        vec
    }
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut res = Self::extract_obj(expression.as_bytes(), 0, expression.len());
        res.sort_unstable();
        res.dedup();
        res
    }
}