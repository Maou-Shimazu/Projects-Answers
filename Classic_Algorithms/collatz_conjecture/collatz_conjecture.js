const even = n => n % 2 === 0;

// not very efficient, but recursion is fun
const collatz = n => {
  if (n === 1) return [1];
  return [n, ...even(n)
    ? collatz(n / 2)
    : collatz(3 * n + 1)];
}

console.log(collatz(42));
