// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


mod macros {
    // 注解表明只要导入了定义这个宏的 crate，该宏就应该是可用的
    #[macro_export] // 宏导出后才能使用
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };   // 宏代码体必须使用 {} 包裹(哪怕仅有单一表达式),且非最末分支必须以" ; "分隔
    }
}

fn main() {
   // 宏必须先声明或导出后 才能使用，有先后顺序
   my_macro!();

   /* error[E0433]: failed to resolve: could not find `my_macro` in `macros`
      macros::my_macro!();
      |            ^^^^^^^^ could not find `my_macro` in `macros` 
   */
   // macros::my_macro!(); // 导入宏 使用 crate_name 而非 module_name

   crate::my_macro!();
}
