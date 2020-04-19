#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod test_mod_1 {
    pub fn do_something() {
        println!("Doing stuff!");
    }

    pub fn more_stuff() {
        foo();
    }

    fn foo() {
        println!("This is function foo!");
    }
}
