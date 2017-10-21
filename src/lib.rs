#![allow(dead_code)]

use std::collections::HashMap;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {

    use MockKDTree;
    use KDTree;
    use SearchableElement;
    use MockEntity;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }



    #[test]
    fn mock_works() {
        let tree = MockKDTree::build(&vec![
            MockEntity {
                id: 1,
                txt: "Apple".to_string(),
            },
            MockEntity {
                id: 2,
                txt: "Banana".to_string(),
            },
            MockEntity {
                id: 3,
                txt: "Lettuce".to_string(),
            },
        ], true);

        let first_result = tree.search("an");
        assert_eq!(first_result.len(), 1);
        assert_eq!(first_result[0].id, 2);
        assert_eq!(first_result[0].index, 1);

        let second_result = tree.search("e");
        assert_eq!(second_result.len(), 2);
        assert_eq!(second_result[0].id, 1);
        assert_eq!(second_result[0].index, 4);
        assert_eq!(second_result[1].id, 3);
        assert_eq!(second_result[1].index, 1);
    }


    #[test]
    fn mock_case_insensitive_works() {
        let tree = MockKDTree::build(&vec![
            MockEntity {
                id: 1,
                txt: "Apple".to_string(),
            },
            MockEntity {
                id: 2,
                txt: "Banana".to_string(),
            },
            MockEntity {
                id: 3,
                txt: "Lettuce".to_string(),
            },
        ], false);

        let first_result = tree.search("A");
        assert_eq!(first_result.len(), 2);
        println!("{:?}", first_result);
        assert_eq!(first_result[0].id, 1);
        assert_eq!(first_result[0].index, 0);
        assert_eq!(first_result[1].id, 2);
        assert_eq!(first_result[1].index, 1);

        let second_result = tree.search("a");
        assert_eq!(second_result.len(), 2);
        println!("{:?}", first_result);
        assert_eq!(second_result[0].id, 1);
        assert_eq!(second_result[0].index, 0);
        assert_eq!(second_result[1].id, 2);
        assert_eq!(second_result[1].index, 1);
    }
}



pub struct MockEntity {
    pub id: u32,
    pub txt: String,
}

impl SearchableElement for MockEntity {
    fn as_searchable_text(&self) -> String {
        return self.txt.to_string();
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}



pub trait SearchableElement {
    fn as_searchable_text(&self) -> String;
    fn get_id(&self) -> u32;
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct SearchResult {
    pub id: u32,
    pub index: usize,
}


impl std::cmp::Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        let idcmp = self.id.cmp(&other.id);
        if idcmp == Ordering::Equal {
            return self.index.cmp(&other.index);
        } else {
            return idcmp;
        }
    }
}



pub trait KDTree {
    fn build<T: SearchableElement>(elements: &Vec<T>, case_sensitive: bool) -> Self;
    fn search(&self, query: &str) -> Vec<SearchResult>;
    fn is_case_sensitive(&self) -> bool;
}

#[derive(Debug)]
pub struct MockKDTree {
    elements: HashMap<u32, String>,
    case_sensitive: bool,
}

impl KDTree for MockKDTree {
    fn build<T: SearchableElement>(elements: &Vec<T>, case_sensitive: bool) -> Self {
        let mut tree = MockKDTree {
            elements: HashMap::new(),
            case_sensitive: case_sensitive,
        };
        for element in elements {
            tree.elements.insert(
                element.get_id(),
                if case_sensitive {
                    element.as_searchable_text()
                } else {
                    element.as_searchable_text().to_lowercase()
                },
            );
        }
        return tree;
    }

    fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = Vec::new();

        let query = if self.is_case_sensitive() {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        for (id, string) in &self.elements {
            let index_opt = string.find(&query);

            if let Some(idx) = index_opt {
                results.push(SearchResult {
                    id: *id,
                    index: idx,
                });
            }
        }
        results.sort();
        return results;
    }
    fn is_case_sensitive(&self) -> bool {
        return self.case_sensitive;
    }
}
