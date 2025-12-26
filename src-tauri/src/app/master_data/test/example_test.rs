struct Example {
    name: String,
}

trait Hello {
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, target: String) -> String;
}
impl Hello for Example {
    fn say_hello(&self) -> String {
        format!("Hello, {}!", self.name)
    }
    fn say_hello_to(&self, target: String) -> String {
        format!("{}, {}", target, self.name)
    }
}

#[test]
fn test_example_test() {
    let exp = Example {
        name: "Example".to_string(),
    };

    println!("{}", exp.say_hello());
    println!("{}", exp.say_hello_to(String::from("Hello World!")));
}

// Contoh struct model query
struct QueryBuilder {
    table_name: String,
    filter: Option<String>,
}

impl QueryBuilder {
    // Memulai query untuk sebuah model
    fn model(name: &str) -> Self {
        Self {
            table_name: name.to_string(),
            filter: None,
        }
    }

    // Menambahkan where clause
    fn where_clause(mut self, condition: &str) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.push_str(" AND ");
            filter.push_str(condition);
        } else {
            let cond = format!(" WHERE {} ", condition);
            self.filter = Some(cond.to_string());
        }

        self
    }

    // Menjalankan query dan mengembalikan hasil (contoh dummy)
    fn find_all(&self) -> Vec<String> {
        let mut result = vec![];
        // Dummy: menambahkan string sebagai contoh hasil
        result.push(format!(
            "Querying table '{}' with filter {:?}",
            self.table_name, self.filter
        ));
        result
    }
}

#[test]
fn test_query_test() {
    let result = QueryBuilder::model("name")
        .where_clause("apa kabar")
        .where_clause("asdasdsadsadsa ")
        .find_all();
    println!("{:?}", result);
}
