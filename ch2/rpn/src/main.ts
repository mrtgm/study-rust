const main = () => {
  const exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
  const ans = rpn(exp);
  console.log(ans);
};

const rpn = (exp: string): number => {
  const stack: number[] = [];
  const tokens = exp.split(" ");
  for (const token of tokens) {
    if (isOperator(token)) {
      const b = stack.pop();
      const a = stack.pop();
      if (b === undefined || a === undefined) {
        throw new Error("Invalid expression");
      }
      stack.push(operate(a, b, token));
    } else {
      stack.push(Number.parseFloat(token));
    }
  }
  const result = stack.pop();
  if (result === undefined) {
    throw new Error("Invalid expression");
  }
  return result;
};

const isOperator = (token: string): boolean => {
  return token === "+" || token === "-" || token === "*" || token === "/";
};

const operate = (a: number, b: number, operator: string): number => {
  switch (operator) {
    case "+":
      return a + b;
    case "-":
      return a - b;
    case "*":
      return a * b;
    case "/":
      return a / b;
    default:
      throw new Error("Invalid operator");
  }
};
