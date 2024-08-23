fn main()
{
    // Signed Integers iKth Bit  
    let x:i8 = 13;
    println!("8Bit Signed Integer {:?}",x);

    let x1:i16 = 4545;
    println!("16Bit Signed Integer {:?} ",x1);

    let x2:i32 = 345345345;
    println!("32Bit Signed Integer {:?}",x2);

    //#TODO: For 64 and 128 Bit Integer There is No Nee To Mention The Bit Types 
    let x3 = 457238462347i64;
    println!("64Bit Signed Integer {:?}",x3);

    let x4 = 2347283472849323423423237i128;
    println!("128Bit Signed Integer {:?}",x4);


    let u1:u8 = 126;
    println!("8Bit Unsigned Integer {0}",u1);

    let u2:u16 = 2800;
    println!("16Bit Unsigned Integer {0}",u2);

    let u3:u32 = 39743; 
    println!("32Bit Unsigned Integer {0}",u3);

    let u4:u64 = 23474644334u64;
    println!("64Bit Unsigned Integer {:?}",u4);

    let u5:u128 = 3427462347862347234823434u128;
    println!("128Bit Unsigned Integer {:?}",u5);

}