pub fn run() {
    // stack over flow を発生させる
    // 8MBを超える配列はstack over flowが発生する

    // まず7MB
    // let a1: [u8; 7000000] = [1; 7000000]; // 1が7,000,000個の配列

    // ---> 結果、下記が発生したが？7MBでもstack overflowが起こるではないか？
    // 「error: process didn't exit successfully: `target\debug\rust-lesson.exe` (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)」

    // どれくらいまでOKか探る
    // まず6.5MB
    // let z1: [u8; 6500000] = [1; 6500000];
    // 結果だめ

    // では5.0MB
    // let z2: [u8; 5000000] = [1; 5000000];
    // 結果だめ

    // では2.0MB
    // let z3: [u8; 2000000] = [1; 2000000];
    // 結果だめ

    // では0.5MB
    // let z4: [u8; 500000] = [1; 500000];
    // 結果OK

    // では1.0MB
    // let z5: [u8; 1000000] = [1; 1000000];
    // 結果OK

    // では1.5MB
    // let z6: [u8; 1500000] = [1; 1500000];
    // 結果だめ

    // とにかく、この講義では、
    // 「スタックには大きな容量のデータを格納できない」
    // と言いたいらしい。

    // ---------------------------------------------
    // ベクター
    // ---------------------------------------------
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("Stack address of v1 is: {:p}", &v1); // &でアドレスを取得
    println!("Stack address of v2 is: {:p}", &v2);

    // Vectorは3層構造（データ領域のポインタ、length、キャパシティ）になっている。
    println!("Heap memory address of v1 is: {:?}", v1.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());

    // Vectorは要素を追加・削除できる。
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);

    // 連結
    println!("{:?}", v3);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3); // v3が空になる！
}
