let age = 1;
let name = "Topal";
let result = 10 * (20 / 2);
let my_array = [1, 2, 3, 4, 5];
let my_hash = {"name": "Axel", "age": 27};
let first = my_array[0];
let name = my_hash["name"];
let add = fn(a, b) { a + b };
let res = add(27, 42);

let fibonacci = fn(n) {
    if (n <= 1) {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
};

let twice = fn(f, x) {
    f(f(x))
};

let seven = twice(fn(x) {x + 2}, 3);