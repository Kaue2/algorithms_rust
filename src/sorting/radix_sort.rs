// complexidade O((n + b) * logb(k)) onde N é o número de elementos
// b é a base e k é o maior elemento
// quando n e b possuem magnetudes próximas o algoritmo é linear
//
// complexidade de espaço é O(n + b)
pub fn radix_sort(arr: &mut [u64]) {
    let max: usize = match arr.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };
    let radix = arr.len().next_power_of_two();

    let mut place = 1;
    while place <= max {
        let digit_of = |x| x as usize / place % radix;

        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }

        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }
        place *= radix;
    }
}
