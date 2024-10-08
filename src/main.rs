use rand::Rng;
use std::fs::File;
use std::io::ErrorKind;
use std::num::FpCategory;
use std::ops::Add;

use std::thread::sleep;
use std::time::Duration;

//-----------Oracle start------------
use oracle::{Connection,Error,Result,Version};
//-----------Oracle   end------------

//-----------Libra start------------
#[macro_use]
extern crate assert_matches;

use assert_approx_eq::assert_approx_eq;
//-----------Libra   end------------



//常量(const)，全局变量(static)的变量名需要大写，不然就会有警告提示，同时需要指定变量类型，编译器不推导，声明时必须初始化值。
//常量一般在实现上使用内联方式，在不同地方对同一常量的引用并不能保证引用到相同的内存地址。
//全局变量则是同一内存空间。
//无法通过mut使常量可变，全局变量则可以使用mut(但必须在unsafe中改变)
const NON_MUT_CONST: u32 = 100_000;
static mut MUT_STATIC: u32 = 100_000;

fn main() {

    unsafe { MUT_STATIC = 12 ;};

    //let x = &var("aa").unwrap();

    /*
    复习点：
    不可变 和 可变[mut]
    数据传递[赋值，函数参数] 涉及 Copy[不可变引用，函数指针...] 和 Move[可变引用，String，Vec...]
    Copy，两个都完全不相关了
    Move，一个丧失了对数据的控制权，另一个则获得了控制权，
    所以为了编程方便，引入引用的概念，引用不涉及所有权的转移
    切片是引用的一种，不涉及所有权的转移
     */

    //test_base();
    test_tuple();
    //test_ref();
    //test_str();
    //test_slice();
    //test_ary();
    //test_match();
    //test_enum();
    //test_struct();
    //test_trait();
    //test_common();
    //test_panic();
    //test_trait();
    //test_closure();

    //test_float();
    //test_libra();

    //test_drop();
    //test_oracle();
    //test_tmp();
}


//基础
fn test_base() {
    /*
        Rust基本类型：这些类型的变量都存储在栈上（这句话脑子记一下，以后会很有用）
        整数类型：u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, [isize,usize--这两个变量占用空间跟机器字长有关] (u表示无符号的） 默认是i32(4个字节)
        浮点类型：f32, f64   默认是f64
        布尔类型：bool（false，true） 1个字节
        字符类型：char 4个字节
        每个变量必须被合理初始化之后才能被使用，除了(unsafe内)
    */
    {
        let n = 5;    //编译器会自动推导n的默认类型
        let n: i32 = 50;   //显式指明变量类型，可以不指明，一般不用显示指明，看下面就知道了
        let n: i64 = 500;
        let n: usize = 5000;
        n + 1;

        let f = 2.;
        let f: f32 = 2.0;
        let b = false;
        let c = '😻';    //unicode字符，4个字节
        let c = '\u{1F600}';    //unicode字符，4个字节
        let u: u8 = b'c';     //此处如果指定类型必须是u8，ASCII字符可以用这个，减少空间, 1个字节
        let u= b'z';

        //整数字面常量，_作为分隔符便于阅读
        let z  = 23_22;         //10进制
        let z = 12_01_i32;
        let z = 0x_ff_i64;      //16进制，这种写法最合理清楚
        let z= 0o_11;           //8进制
        let z = 0b_11_00_i32;   //2进制

        //后缀形式(整数类型和浮点数类型都可以使用后缀形式表达，其他的不可以哦)，Rust要求类型后缀之前不能有分隔符
        let n = 23i32;
        let f = 0.23f32;
        let u = b'a';   //只有这一种u8整数类型的变量没有办法使用后缀形式表达

        //一次为多个变量进行赋值，猜一猜他们各自是什么类型
        let (n, f, b, u, c) = (34u32, 2.4, true, b'k', '😻');
        println!("{},{},{},{},{}", n, f, b, u, c);     //println!不是一个函数，是一个宏


        // 类型转换必须是显式的，Rust不会自动转换
        // 只有同样类型，才能运算
    }

    /*-------------------------------------------------------------------------------------------------------*/
    /*----------以上只要关心基本类型和后缀表达，其他不要纠结，后面会讲解，整理思绪，take it easy ！----------*/
    /*-------------------------------------------------------------------------------------------------------*/

    /*
        重要知识点：
        不可变变量和可变变量---变量默认不可变
    */
    {
        let x;
        x = 3;  //此处是初始化，不是修改

        //以下是x变量遮蔽，内存空间完全不同，类型也可以不同
        let x = 5;  //x为默认不可变变量
        //x = 6;    //Error，因为x是不可变的，也就是说x无论在哪儿都是只读的，只能使用无法修改。除非重新定义x  （Rust语言的安全性开始体现了）
        let x = false;
        //x = true; //Error，一样是不可变原理

        let mut x;
        x = 5i32;

        let mut x = 5;  //mut关键字，x重新定义为可变变量，可变变量可以修改值，但类型必须一致，除非重新定义x
        x = 34;
        x = 23u8;
        //x = 34u64;    //Error，类型不一致 , 基本类型通过as转换
        //x = 'c';      //Error，类型不一致

        let mut x: char = 'c';
        x = 'u';
        //x = b'u';     //Error，类型不一致

        let mut x = false;
        x = true;

        fn test_immutable(y: i32) {
            //此括号内是形参y的作用域，在此作用域内，y就是一个普通的不可变的变量
            //y = 32;   //Error，思考一下是什么原因呢？
        }
        fn test_mutable(mut y: i32) {
            y = 13;
            println!("y is {}",y);
        }

        let x = 32;
        test_mutable(x);
        println!("x is {}", x);
        //此处x没必要一定是可变变量
        //如果从栈变量的值Copy来看，由于发生了Copy，原先变量x是否可变不重要了，拷贝后的变量y是否可变是关键
        //原参和形参一旦发生Copy，就没有关联了
    }

    /*
        重要知识点：
        Copy（拷贝）和Move（所有权转移）
    */
    {
        //以下就是Copy，第一这些变量是在栈上的，拷贝非常的快，不涉及所有权的转移，主要原因是基础类型都实现了Copy trait
        let x = 5;
        let y = x;

        let x = true;
        let y = x;
        //y = false;    //Error，因为y是不可变的

        {
            //思考一下，Copy的相关细节
            let x = 23;
            let mut y = x;
            let z = y;
            println!("{:p},{:p},{:p}", &x, &y, &z);     //不同的内存地址
        }// 在这个点，x、y、z 都已经被释放了 {} 作用域结束了

        fn test_stack(i: bool) {
            //i = false;    //Error，思考一下为什么哦
            println!("{}", i)
        }
        test_stack(y);    //此时也是参数变量的值拷贝，（各自还是各自，互不干扰，只不过是值相同而已）
        println!("{},{}", x, y);


        //引入第一在堆上的变量
        let s = String::from("heap");
        let s1 = s;     //此处就不是Copy了，此处是Move，所有权发生了转移，s被丢弃将无法使用，除非重新定义s
        //println!("{}",s);     //Error，s已经被丢弃了，丧失了对"heap"的所有权

        //重新定义了s，此时s与s1互不相关了
        let s = String::from("new heap");
        println!("{},{}", s,s1);
        fn test_heap(s: &String) {
            println!("{}", s);
        }
        test_heap(&s);

        fn test_heap1(s: String) {
            println!("{}", s);
        }
        test_heap1(s);  //此处也不是参数变量的值拷贝了，是Move，是所有权发生了转移，转移到了test_heap函数的变量中，s被丢弃了将无法使用，除非重新定义s或者返回出来
        //println!("{}",s);     //Error，s已经被丢弃了，丧失了对"new heap"的所有权

        /*
            总结：
            因为栈上的变量Copy速度很快，所以就各自拥有各自的所有权，不会有所有权的转移
            堆上的变量的操作速度相对慢很多，体量又有可能很大，因此不拷贝，所以就有了所有权的转移[除非使用引用，后面会讲到]。所有权的转移也是体现了Rust语言的安全性
        */
    }
}

//元组
fn test_tuple() {
    //元组在声明时就必须确定各个值(类型可以不同)，且一旦声明结束，空间大小就是固定的，后续不能再增大或减小
    {
        let x: (i32, char, bool) = (23, 'a', true);
        let x = (12i8, 7.88, String::from("tuple"));
        let (a, b, c) = x;  //此处有变量是Copy（a,b）,有变量是Move(c),所以x只是被部分Move了，其余[x.]还可以使用，x中指向c的指针已经失效了
        println!("{},{},{},{},{}", a, b, c, x.0, x.1);
        //println!("{}",x.2);   //Error，变量x.2已经Move了
        //println!("{:?}",x);   //Error，变量x由于已经被部分Move了，所以也无法再单独使用了

        let x = (12i8, 7.88, String::from("tuple"));
        let y = x;
        //println!("{},{}", x.0, x.1);  //Error，变量x已经Move了
    }

    {
        //Rust 的所有权系统在处理复合类型时的一致性
        let x = (12i8, 7.88, true);
        let y = x;   //由于此元组元素是基本类型存储在栈上，所以是全量Copy，没有所有权的转移, 相当于新建了个副本互不干扰

        let x = (12i8, 7.88, String::from("new tuple"));
        let y = x;  //此时由于有堆上数据，所以变量x已经被完全Move了，但是其中的基本类型并没有复制
        //println!("{},{}", x.0, x.1);  //Error，想一想 [整体赋值，所有权发生了完全的转移]
    }

    {
        let x = (45, true, 'c');
        fn test_copy(tup: (i32, bool, char)) {
            //只读，无法修改
        }
        test_copy(x);
        print!("{:?}", x);

        let x = (45, true, String::from("tuple"));
        fn test_move(mut tup: (i32, bool, String)) {
            tup.0 = 23;
            tup.1 = false;
            tup.2 = String::from("mut tuple");
        }
        test_move(x);
    }

    {
        //注意：元组也满足不可变和可变性
        let mut x = (32, false, String::from("mut tuple"));
        x.0 = 34;
        x.2.push_str(" push");
        x = (45, true, String::from("rust"));
        fn test_mut(mut tup: (i32, bool, String)) {}
        fn test_ref_mut(tup: &mut (i32, bool, String)) {}
    }
}

//引用---避免所有权的转移[要尊重原数据]--借用行为
fn test_ref() {
    {
        //基础类型（在栈上赋值很快，一般不用引用）[可以有多个不可变引用（&T）或一个可变引用（&mut T），但这两种引用不能同时存在。]
        let mut x = 5;
        let y = &mut x;
        let z = y;
        //*y = 7;   //Error，y的所有权已经转移给z了，y已经失效了
        println!("{}", *z);

        let x = 32;
        let y = &x;
        println!("{:p},{:p}", &x, y);
        println!("{},{}", x, y);


        //引用不涉及到所有权的Move
        let x = String::from("Str");
        let y = x;
        //println!("{},{}", x, y);  //Error，x所有权Move了

        let x = String::from("refStr");
        let y = &x;
        println!("{:p},{:p}", &x, y);
        println!("{},{}", x, y);
    }


    {
        let mut x = String::from("ref");
        let mut z = String::from("newRef");
        let _x1 = &x;
        x.push_str("Str");
        {
            //由于有下面可变引用y的存在
            //所以，只有此范围可以对x1进行只读操作

            //x1.push_str("tt");    //Error
            x.push_str("Str");
            //println!("{}", x1); //Error，x1已经失效了无法被用
        }

        let y = &mut x;     //此处开始，则前面涉及x的相关引用（包括x）也将无法再被使用，直到y不被使用了
        //x.push_str("");   //Error
        //y = &mut z;   //Error
        y.push_str("aa");
        println!("{}", x);
        x.push_str("bb");
        //println!("{}", y);    //Error，可变引用的使用在不可变引用的使用之后，这是不行的
        let mut y = &mut x;
        y.push_str("bb");
        //以下一旦使用了x，y只能被重新赋值后才能操作
        println!("{}", x);
        //y.push_str("cc"); //Error
        y = &mut z;
        println!("{}", x);
        let t= &mut x;
        y.push_str("mut");
        //let y1 = &mut z;  //同时定义两个可变引用不会有错，但是用的时候就会编译不通过(因为可能会同时改变同一块内存)，所以不要同时声明两个同样的可变引用
        test_mutable_ref(y);

        fn test_mutable_ref(str: &mut String) {
            str.push_str("go");
        }
        println!("{}", y);  //此处之后可以恢复z的使用

        println!("{}", z);  //y的使用范围到此处截止，y只能被重新赋值后才能操作，否则无法使用了
        //y.push_str("dd"); //Error



        t.push_str("tt");

        /*
            总结：
                1. 可变引用的写法
                2. 两大限制：不要同时存在两个可变引用；存在一个可变和一个不可变时，使用范围有限制（后面的会导致前面的不可用）
        */
    }
}

//字符串
fn test_str() {
    //`str` 是一个动态大小类型（DST），不能直接使用，总是通过引用（`&str`）使用。
    //`&str` 是不可变的。如果需要可变字符串，应使用 `String`。
    //理解和正确使用 `str` 和 `&str` 对于编写高效的 Rust 代码非常重要。它们提供了一种内存安全且高效的方式来处理字符串数据。
    let s = String::from("Libra");
    s.len();
    s.capacity();
    let s1 = s.clone();    //深拷贝

    let mut s_r = "Libra";  //字符串字面常量其实也是个引用
    s_r.len();
    s_r = &s;   //&String可以被强制转换成&str，反则不可以

    //&str => String
    let a = "RustGo";
    let a1 = a.to_string();
    let a2 = String::from(a);
    let a3 = a.to_owned();


    //String => &str
    let a = String::from("RustGo");
    let a1 = &a;    //一个&String可以当做是&str
    let a2 = a.as_str();


    let f1 = 2.3;
    let i1 = 5;
    let com_str = format!("{}--{}--{}", f1, i1, "zz");     //这种组合构造方式不错哦

    fn create_greeting(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    //如果只想要一个字符串的只读视图，那就首选&str。如果想拥有所有权，想修改字符串那就用String吧。
    let tt: &str;
    fn normal(s: &str) {}   //参数多数喜欢使用&str而不是&String
}

//切片
fn test_slice() {
    //切片是一种特殊的引用,允许引用集合类型的部分元素,由于是引用所以没有（所有权Move）的概念,但是还是有（可不可变）的概念的
    //String Slice
    {
        let s = String::from("hello rust");
        let s0 = "hello rust";      //字符串字面常量其实也是个引用
        let s1 = &s[0..5];          //[0-5)
        let s2 = &s[..3];           //[0-3)
        let s3 = &s[..=3];          //[0-3]
        let s4 = &s[3..s.len()];
        let s5 = &s[3..];
        //let s5 = s0[3..];     //Error,写法错误
        println!("{},{},{},{},{},{},{}", s, s0, s1, s2, s3, s4, s5);

        //String + &str => String , String后面可以接上N个&str
        let s6 = String::from("hello rust");
        //let s7 = s + s6;           //Error
        //let s7 = s0 + s6;          //Error,字符串加法必须以String类型开头，后面相加的必须是&str
        let s7 = s + &s6 + s0 + " end";   //此处编译器会将&String类型强制转换成&str，同时s的所有权Move了
        //println!("{}",s);         //Error,s被丢弃了
        println!("{}", s7);

    }

    //Array Slice
    {
        //DST 动态大小类型
        let ary = [12, 13, 14, 15, 16, 17];

        let ary1 = &ary[2..5];
    }
}

//数组
fn test_ary() {
    /*
    使用array的场景：
        1.元素个数确定时，例如一年的12个月的名称
        2.希望数据分配在栈上而不是堆上时(以下有数组元素在堆上的例子)
    */
    {
        //数组类型必须一致,且它们的大小在编译时会被确定,没有办法增加和减少元素
        //一维数组
        let ary = [12, 13, 14];
        let ary: [f64; 5] = [23.4, 45.6, 78.5, 34.5, 12.3];
        let ary: [f64; 5] = [2.3; 5];     //对5个数初始化2.3
        //ary[0] = 9.1;     //Error,不可变数组
        //let a = ary[10];    //Error,超出索引范围
        let ary1 = ary; //Copy
        println!("{:?}", ary);
        //与 {} 不同，{:?} 不要求类型实现 std::fmt::Display trait
        //{:#?} 是 {:?} 的一个变体，它提供了更易读的多行输出格式


        //二维数组
        let ary: [[i32; 3]; 2] = [[5; 3]; 2];
        let ary: [[i32; 3]; 2] = [[5, 5, 5], [5, 5, 5]];  //和上面一样
    }

    {
        let ary = [String::from("Boo"), String::from("Eoo"), String::from("Libra")];
        //let ary = [String::from("Etc"); 5];   //Error,元素没有Copy的特性
        let ary1 = ary; //Move
        //println!("{:?}", ary);    //Error,所有权转移了

        let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
        println!("{:#?}", array);
    }

    {
        let mut ary = [5; 3];    //只可以修改值，不能删除和增加
        ary[0] = 10;
    }

    {
        let ary = [String::from("Boo"), String::from("Eoo"), String::from("Libra"), String::from("Etc")];
        let ary1 = &ary[1..3];
        fn test_ref(ary: &[String]) {
            return;
        }
        fn test_ref_len(ary: &[String; 2]) {
            return;
        }
        fn test(ary: [String; 4]) {
            return;
        }
        test_ref(&ary);
        test_ref(ary1);
        test(ary);  //Move
        //test_ref_len(&ary);  //Error,无法匹配
        //test_ref_len(ary1);  //Error,无法匹配
        let ary = [String::from("Boo"), String::from("Eoo")];
        test_ref_len(&ary);
    }

    {
        //复杂数组
        let player_scores = [("shilf",90),("shilp",87),("shily",69)];
    }
}

//匹配
fn test_match() {
    //匹配--可以让你有效的取代复杂的if/else组
    //强制穷尽性检查或者使用_通配符或者使用一个变量来承载其他情况比如这里的z
    //当你只关心一种情况时，可以使用 if let 替代 match
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Get Now !"),
        4 => println!("four"),
        5 => println!("five"),
        z => println!("something else {}",z),
    }   //此处理论上可以被_,5,z捕获，但是先来先到，所以被_通配符捕获了

    //if let 相当于其中一个match的简略形式
    if let 5 = x {
        println!("five!");
    }else{
        println!("something else");
    }

    let r = rand::thread_rng().gen_range(1..100);
    let n = if r > 50 {
        r - 50
    } else {
        r + 50
    };

    match File::open("xx.txt") {
        Ok(_) => println!("open success"),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(_) => println!("create success"),
                Err(e) => println!("create fail {:#?}", e),
            },
            other_err => println!("open fail {:#?}", other_err),
        },
    }

    //matches!宏--研究一下
}

//枚举
fn test_enum() {
    #[derive(Debug,Copy,Clone)]
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }
    let clubs = PokerSuit::Clubs;
    let x = clubs;  //此处是Copy
    let hearts = PokerSuit::Hearts;
    let diamonds = PokerSuit::Diamonds;
    let spades = PokerSuit::Spades;

    println!("{:?}-{}", clubs,clubs as i32);
    println!("{:?}-{}", hearts,hearts as i32);
    println!("{:?}-{}", diamonds,diamonds as i32);
    println!("{:?}-{}", spades,spades as i32);



    #[derive(Debug)]
    enum MyLanguage {
        Java(i32),
        Rust(i32),
        Python,
        C(i32, String),
        //元组
        Julia { version: i32, name: String }, //匿名元组结构体
    }
    let la1 = MyLanguage::Java(1);
    let x = la1;    //此处la1的所有权Move给了x
    let la2 = MyLanguage::Java(2);
    let la3 = MyLanguage::Rust(1);
    let la4 = MyLanguage::Python;
    let la5 = MyLanguage::C(1,"C".to_string());
    let la6 = MyLanguage::Julia{version:1,name:"Julia".to_string()};
    println!("{:?}",x);
    println!("{:?}",la2);
    println!("{:?}",la3);
    println!("{:?}",la4);
    println!("{:?}",la5);
    println!("{:?}",la6);



    impl MyLanguage{
        fn findLanguage(&self){
            match self {
                MyLanguage::Rust(version) => println!("{}", version),
                MyLanguage::C(_, _) => (),
                other_language_version => println!("{:?}", other_language_version),
            }
        }
    }

    MyLanguage::Java(7).findLanguage();
    MyLanguage::findLanguage(&MyLanguage::Java(7));


    //if let替代match的表现形式
    let l_rust = MyLanguage::Rust(2);
    if let MyLanguage::Python = l_rust {
        println!("Python!");
    } else {
        println!("Other Language!");
    }
}

//结构体
fn test_struct() {
    struct Point{
        x: i32,
        y: i32,
        z: f32
    }   //Rust 不支持将某个结构体某个字段标记为可变

    //初始化实例时，每个字段都需要进行初始化
    //初始化时的字段顺序不需要和结构体定义时的顺序一致

    //要可变只能这样
    let mut _p = Point{x :0, y :23, z :0.32};

    let p = Point{x :0, z :0.32, y :23};
    let p1 = Point{z :0.46, ..p};    //其他都一样

    let x = 1;
    let y = 2;
    let z = 2.3;
    let mut p = Point{x, y, z};     //同名可以省略key，否则形式上必须是key:value
    p.x = 1;    //此处只能.key,不可以.0

    //生命周期结构体
    struct  PointRef<'a>{
        x: &'a mut i32,
        y: &'a mut i32,
    }
    let mut a = 1;
    let mut b = 2;
    let p1 = PointRef{ x: &mut a, y: &mut b};
    *p1.x = 3;
    *p1.y = 4;
    println!("{} , {}",a,b);

    struct Car {
        name: &'static str,
        age: i32,
    }


    //匿名成员结构体,又叫“元组结构体”
    struct Point2(i32, i32);
    let p = Point2(23, 45);
    let x = p.0;
    let y = p.1;
    let Point2(_, z) = p;


    #[derive(Debug)]  //这样才能使用 println!("{:?}",user)
    struct User {
        name: String,
        age: i32,
    }

    struct Empty{}
    struct Empty1;

    //方法调用--self表示调用方法的对象，Self表示调用者的类型
    //参数self 等价于 self : Self
    //参数&self 等价于 self : &Self
    //参数&mut self 等价于 self : &mut Self
    impl User {
        //Self指的是调用者的类型
        pub fn new(name: String, age: i32) -> Self {
            User { name, age }   //此处不能有分号，同名可以省略key，否则形式上必须是key:value
        }
        pub fn new1(name: String, age: i32) -> User {
            User { name, age }
        }

        //&self指的是类对象
        fn change_user_name(&mut self, new_name: String) {
            self.name = new_name;
        }

        //等价于fn isLarger(&self, other : &User) -> bool
        fn isLarger(&self , other : &Self) -> bool{
            self.age > other.age    //此处不能有分号，有分号就不是返回语句了
        }

        fn xx(){}

        //总结：参数里面没有self的就是静态方法(关联函数)，需要类型调用
    }


    let mut user = User { name: String::from("shilf"), age: 41 };
    let mut user = User::new(String::from("shilf"), 32);
    let mut user = User::new1(String::from("shilf"), 32);
    let user1 = User { name: String::from("shily"), ..user };      //此处只是做了一次复制，没有任何关联关系
    user.change_user_name(String::from("shilp"));
    user.name.push_str("--yx");
    User::change_user_name(&mut user,String::from("shijh"));
    User::xx();
    user.isLarger(&user1);
    println!("{:#?}", user);
    println!("{:#?}", user1);
}

fn test_vc(){
    let mut v : Vec<i32> = Vec::new();
    v.push(12);

    let mut v = Vec::new(); //推导类型
    v.push(23);

    let mut v = vec![2,3,4];    //宏+推导类型
    v.push(34);

    let mut v = Vec::<i32>::new();  //turbofish语法
    v.push(45);
}

//泛型
fn test_generic() {
    //枚举中的泛型
    let x : Option<i32> = Some(5);
    let x : Option<f64> = Some(2.3);
    let x : Option<bool> = Some(true);

    //普通的函数泛型
    fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
        a + b
    }

    //结构体泛型
    struct Point<T>{
        x : T,
        y : T,
        z : T
    }
    let p1 = Point { x: 1, y: 2, z: 3 }; 

    struct Point1<T = i32>{
        x : T,
        y : T,
        z : T
    }
    let p1 = Point1{x: 2, y: 3, z: 4};

    //方法定义中的泛型
    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }

}

//特性
fn test_trait() {
    //类似其他语言的接口，要想实现特性，必须实现特性中的所有方法（以实现的方法除外，列如xx）
    trait FT {
        fn format(&self) -> String;

        //fn xx(){}  //如果加这种第一参数不是self相关的方法，就没有办法动态分发，原因是trait不是对象安全的
    }

    struct CC;
    impl CC{
        fn format(&self) -> String {
            return String::from("CC");
        }
    }

    impl FT for CC {
        fn format(&self) -> String {
            return String::from("FT::CC");
        }
    }
    let c = CC{};
    println!("{}", c.format());
    println!("{}", <CC as FT>::format(&c));
    println!("{}", FT::format(&c)); //缩写模式


    impl FT for u8 {
        fn format(&self) -> String { format!("{}:u8", *self) }
    }

    impl FT for f64 {
        fn format(&self) -> String { format!("{}:u8", *self) }
    }

    impl FT for String {
        fn format(&self) -> String { format!("{}:String", *self) }
    }

    let x = b'a';
    FT::format(&x);     //关联函数用法，和下面等价
    println!("{}",x.format());

    //泛型的特性范围
    //静态分发--缺点是代码会膨胀，但运行时没有开销
    fn printX<T : FT>(tmp : &T) {
        println!("{}", tmp.format());
    }
    //动态分发--缺点是调用虚函数，但运行时有开销（虚函数表）
    fn printY(tmp: &dyn FT) {
        println!("{}", tmp.format());
    }

    fn printZ(tmp: Box<dyn FT>) {
        println!("{}", tmp.format());
    }

    /*
    fn printZ(tmp: FT) {
        println!("{}", tmp.format());
    }*/     //Error,FT的编译器Size不确定，所以要使用指针

    struct Point<T : FT>{
        x : T,
        y : T,
        z : T
    }

    impl<T:FT> FT for Point<T> {
        fn format(&self) -> String {
            format!("Point: x={} , y={} , z={}", self.x.format(), self.y.format(), self.z.format())
        }
    }

    let p = Point{x:b'a', y:b'c', z:b'g'};
    printX(&p);
    printY(&p);

    //多限定，特性继承，泛型特性
    trait Large<T> : FT{
        fn isLarger(&self , other: T) -> bool;
    }
    //fn tt<T:Large>(x: T){}    //Error:文法错误
    fn tt<T:Large<u8>>(x: T){}
    fn tt1<T:Large<T>>(x: T){}

    impl Large<u8> for u8{
        fn isLarger(&self, other: u8) -> bool{
            *self > other
        }
    }
    impl Large<f64> for f64{
        fn isLarger(&self, other: f64) -> bool{
            *self > other
        }
    }
    impl Large<u8> for f64{
        fn isLarger(&self, other: u8) -> bool{
            *self > other as f64
        }
    }

    b'a'.isLarger(b'c');

    trait Small<T,E> {
        fn isSmaller(x: T , y: E) -> bool;
    }

    fn zz<T : Large<T>,K : Large<T> + Small<T, K>>(x: T, y: K){
        println!("zz");
    }

    fn zz1<T,K>(x: T, y: K)
        where T: Large<T>,
              K: Large<T> + Small<T, K>{
        println!("zz1");
    }
}

fn test_drop() {
    struct Firework {
        strength: i32,
    }

    impl Drop for Firework {
        fn drop(&mut self) {
            println!("Drop {}!!!", self.strength);
        }
    }

    let a = Firework { strength: 1 };
    let b = Firework { strength: 100 };
    //后进先出原则
}

fn test_common() {
    //statement(语句)和expression(表达式)的关系，statement执行一些动作但不会得到一个可返回的值，expression会得到一个可返回的结果值
    //statement是以分号结束的，但expression没有结尾的分号

    //rust没有(a > 0 ? true : false)这种条件表达式
    //rust的条件表达式，条件必须是bool类型
    let n = if 20 > 0 {
        1
    } else {
        2
    };


    fn noReturn1(x:i8) -> i8
    {
        if x > 0 {
            0
        }else {
            1
        }
    }

}

fn test_panic() {
    panic!("crash and exit");
}

//宏
fn test_macro() {
    macro_rules! ttg {
        () => {};
    }
}

//闭包
fn test_closure() {
    let mut x = 4;
    let mut change = |mut v| {
        x = 1;  //对于此处x而言，相当于是&mut x，由于要修改x，所以change是FnMut需要加mut。
        v = x;  //对于此处x而言，是Copy，由于v是可变的，所以是mut v
        v
    };

    let y = 10;
    let v = change(y);  //此处y是Copy

    let cc = |v: &mut i32| {
        *v = 10;
        *v
    };
}

fn test_control(){
    let a = [1,2,3,4,5];
    for e in a.iter(){
        println!("the value is: {}", e);
    }
}

fn test_float() {
    // 浮点数一共有5个种类，在std::num::FpCategory下面定义了
    // 由于FpCategory::Nan的存在，浮点数是偏序关系，不是全序关系，无法在集合内比较大小
    let int_vec = [1_i32, 2, 3];
    let biggest_int = int_vec.iter().max();
    let float_vec = [1.0_f32, 2.0, 3.0];
    //let biggest_float = float_vec.iter().max();   //Error

    //让大家感受一下浮点数有哪5个种类
    let x = 1.0f32 / 0.0;
    let y = 0.0f32 / 0.0;
    println! ( "{} , {}", x, y);
    assert_eq!(x.classify(), FpCategory::Infinite);
    assert_eq!(y.classify(), FpCategory::Nan);

    let mut small= std::f32::EPSILON;
    while small > 0.0 {
        small = small / 2.0;
        if small.classify() == FpCategory::Normal {
            print!("Normal-");
        } else if small.classify() == FpCategory::Subnormal {
            print!("SubNormal-");
        } else if small.classify() == FpCategory::Zero {
            print!("Zero-");
        }
        println!("{} ", small);
    }
}




//---------------------------------Libra----------------------------
fn test_libra() {
    test_assert_approx_eq(2.000000001f64, 2.000000002f64, None);
    //test_assert_approx_eq(2.000000001f64,2.000000002f64, Some(1.0e-16));
}

fn test_assert_approx_eq(x: f64, y: f64, diff: Option<f64>) {
    //判断两个数大致相等，若不大致相等，会panic
    //默认diff是1.0e-6
    match diff {
        None => assert_approx_eq!(x,y),
        _ => assert_approx_eq!(x,y,diff.unwrap()),
    };
}

fn test_assert_match() {}



//---------------------------------Oracle start-------------------------------
fn test_oracle() {
    //把instantclient_11_2文件夹配置到系统环境变量中，确保是64位就可以了
    let conn = Connection::connect("system", "oracle", "10.101.10.41:1521/spdb").unwrap();

    let client_ver = Version::client().unwrap();
    println!("Oracle Client Version: {}", client_ver);

    let (server_ver, banner) = conn.server_version().unwrap();
    println!("Oracle Server Version: {}", server_ver);
    println!("--- Server Version Banner ---");
    println!("{}", banner);
}




//---------------------------------tmp_test-------------------------------
fn test_tmp() {
    {
        let mut arry = [1,2,3,4,5];
        let a1= &mut arry[1..3];
        a1[0] = 10;
        println!("{:?}",arry);
    }

    {
        struct File {
            name: String,
            data: Vec<u8>,
        }
        let mut f1 = File {
            name: String::from("f1.txt"),
            data: Vec::new(),
        };

        let f1_length = &f1.data.len();
        let y = f1.data.len();

        println!("初始长度: f1_length = {}, y = {}", f1_length, y);

        f1.data.push(1);
        f1.data.push(2);

        println!("修改后: f1_length = {}, y = {}", f1_length, y);
        println!("实际长度: {}", f1.data.len());
    }

}
