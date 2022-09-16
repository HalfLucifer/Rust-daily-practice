use std::collections::HashMap;

pub struct MonarchyTree {
    queen: String,
    map: HashMap<String, Person>,
}

impl MonarchyTree {
    pub fn new(queen: &str) -> Self {
        let q = Person::new(queen);
        let mut hm: HashMap<String, Person> = HashMap::new();

        hm.insert(queen.to_owned(), q);

        Self {
            queen: queen.to_owned(),
            map: hm,
        }
    }

    pub fn born(&mut self, name: &str, parent: &str) {
        if let Some(node) = self.map.get_mut(parent) {
            node.children.push(name.to_owned());
            self.map.insert(name.to_owned(), Person::new(name));
        }
    }

    pub fn die(&mut self, name: &str) {
        if let Some(node) = self.map.get_mut(name) {
            node.is_alive = false;
        }
    }

    pub fn revive(&mut self, name: &str) {
        if let Some(node) = self.map.get_mut(name) {
            node.is_alive = true;
        }
    }

    pub fn get_order_of_successors(&self) -> Vec<String> {
        fn get_order_of_successors_recursive(person: &Person, map: &HashMap<String, Person>, result: &mut Vec<String>) {
            // Preorder traversal on N-ary tree
            if person.is_alive {
                result.push(person.name.clone());
            }
    
            person.children.iter().for_each(|name| {
                get_order_of_successors_recursive(map.get(name).unwrap(), map, result);
            });
        }
    
        let mut result = vec![];
        let queen = self.map.get(&self.queen).unwrap();
        get_order_of_successors_recursive(&queen, &self.map, &mut result);

        result
    }
}

struct Person {
    name: String,
    is_alive: bool,
    children: Vec<String>,
}

impl Person {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            is_alive: true,
            children: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        /*
            Elizabeth
            - Charles
              - William
                - George
                - Charlotte
                - Louis
              - Harry
                - Archie
                - Lilibet
            - Anne
              - Peter
                - Savannah
                - Isla
              - Zara
                - Mia
                - Lena
                - Lucas
            - Andrew
            - Edward
        */
        let mut mo = MonarchyTree::new("Elizabeth");

        // 2nd gen
        mo.born("Charles", "Elizabeth");
        mo.born("Anne", "Elizabeth");
        mo.born("Andrew", "Elizabeth");
        mo.born("Edward", "Elizabeth");

        // 3rd gen
        mo.born("William", "Charles");
        mo.born("Harry", "Charles");

        mo.born("Peter", "Anne");
        mo.born("Zara", "Anne");

        // 4th gen
        mo.born("George", "William");
        mo.born("Charlotte", "William");
        mo.born("Louis", "William");

        mo.born("Archie", "Harry");
        mo.born("Lilibet", "Harry");

        mo.born("Savannah", "Peter");
        mo.born("Isla", "Peter");

        mo.born("Mia", "Zara");
        mo.born("Lena", "Zara");
        mo.born("Lucas", "Zara");

        let mut expected = vec![
            "Elizabeth", "Charles", "William", "George", "Charlotte", "Louis",
            "Harry", "Archie", "Lilibet", "Anne", "Peter", "Savannah", "Isla",
            "Zara", "Mia", "Lena", "Lucas", "Andrew", "Edward"];
        assert_eq!(mo.get_order_of_successors(), expected);

        mo.die("Elizabeth");
        expected.remove(0);
        assert_eq!(mo.get_order_of_successors(), expected);

        mo.die("Charles");
        expected.remove(0);
        assert_eq!(mo.get_order_of_successors(), expected);

        let names = ["William", "George", "Charlotte", "Louis"];
        for n in names.iter() {
            mo.die(n);
            expected.remove(0);
        }
        assert_eq!(mo.get_order_of_successors(), expected);

        let names = [
            "Harry", "Archie", "Lilibet", "Anne", "Peter", "Savannah", "Isla", "Zara", "Mia",
            "Lena", "Lucas", "Andrew",
        ];
        for n in names.iter() {
            mo.die(n);
        }
        assert_eq!(mo.get_order_of_successors(), ["Edward"]);
    }

    #[test]
    fn test_edge_cases() {
        let mut mo = MonarchyTree::new("Elizabeth");
        assert_eq!(mo.get_order_of_successors(), ["Elizabeth"]);

        mo.born("Huh", "Who");
        assert_eq!(mo.get_order_of_successors(), ["Elizabeth"]);

        mo.die("Elizabeth");
        let expected: Vec<&str> = vec![];
        assert_eq!(mo.get_order_of_successors(), expected);
    }
}
