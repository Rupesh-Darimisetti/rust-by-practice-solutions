// 1. 🌟🌟 <T, const N: usize> is part of the struct 
// type, it means Array<i32, 3> and Array<i32, 4> are different types.


struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 5]
        }
    ];

    println!("Success!");
}
