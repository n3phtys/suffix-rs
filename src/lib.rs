#![allow(dead_code)]

use std::collections::HashMap;


#[cfg(test)]
mod tests {

    use MockKDTree;
    use KDTree;
    use SearchableElement;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    struct MyEntity {
        id: u32,
        txt: String,
    }

    impl SearchableElement for MyEntity {
        fn as_searchable_text(&self) -> String {
            return self.txt.to_string();
        }

        fn get_id(&self) -> u32 {
            return self.id;
        }
    }


    #[test]
    fn mock_works() {
        let tree = MockKDTree::build(&vec![MyEntity{
            id: 1, txt: "Apple".to_string()
        }, MyEntity{
            id: 2, txt: "Banana".to_string()
        }, MyEntity{
            id: 3, txt: "Lettuce".to_string()
        }]);

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
}


pub trait SearchableElement {
    fn as_searchable_text(&self) -> String;
    fn get_id(&self) -> u32;
}

pub struct SearchResult {
    id: u32,
    index: usize,
}

pub trait KDTree {
    fn build<T: SearchableElement>(elements : &Vec<T>) -> Self;
    fn search(&self, query: &str) -> Vec<SearchResult>;
}

pub struct MockKDTree {
    elements : HashMap<u32, String>,
}

impl KDTree for MockKDTree {
    fn build<T: SearchableElement>(elements: &Vec<T>) -> Self {
        let mut tree = MockKDTree {
            elements: HashMap::new()
        };
        for element in elements {
            tree.elements.insert(element.get_id(), element.as_searchable_text());
        }
        return tree;
    }

    fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results : Vec<SearchResult> = Vec::new();

        for (id, string) in &self.elements {

            let index_opt = string.find(query);

            if let Some(idx) = index_opt {
                results.push(SearchResult {
                    id: *id,
                    index: idx,
                });
            }
        }

        return results;
    }
}