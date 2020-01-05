mod my_mod {
    fn private_function() {
        println!("called my_mod::private_function()");
    }
    pub fn function() {
        println!("called my_mod::function()");
    }

    pub fn indirect_access() {
        print!("Called my_mod::indirect_access(), that ");
        private_function();
    }

    pub fn call_public_function_in_my_mod(){
        print!("called my_mod::call_public_function_in_my_mod(), that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub (crate) fn public_function_in_crate(){
        println!("called my_mod::public_function_in_crate()");
    }

    // modules can be nested
    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called my_mod::nested::public_function_in_my_mod(), that\n>");
            public_function_in_nested();
        }

        // it is visible within current module
        pub(self) fn public_function_in_nested(){
            println!("called my_mod::nested::public_function_in_nested()");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod()");
        }
    }

    mod private_nested {
        pub fn function(){
            println!("called my_mod::private_nested::function()");
        }

        pub(crate) fn restricted_function(){
            println!("called my_mod::private_nested::restricted_function()");
        }
    }
}

fn function() {
    println!("called function()");
}
fn main() {
    function();
    my_mod::function();
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();

    //Below fn is public only in specified path otherwise private
    //my_mod::nested::public_function_in_my_mod();

    //my_mod::private_function();
    
    //my_mod::nested::private_function();

    //private module
    //my_mod::private_nested::function();
    //my_mod::private_nested::restricted_function();

}