// Calculator repl
#include <assert.h>

#include <iostream>
#include <stack>
#include <string>
#include <tuple>
#include <unordered_map>
#include <math.h>

using namespace std;

enum TokKind {
  NUM = 256,
  ERROR,
  EOE,
};

struct Tok {
  int kind;
  string text;
};

// Current token
Tok ctok;

struct Expr {
  string& s;
  int ptr = 0;
};

unordered_map<char, int> BinOpPrec = {
    {'^', 3}, {'*', 2}, {'/', 2}, {'+', 1}, {'-', 1},
};

void lex(Expr& E) {
  if (E.ptr >= E.s.length()) {
    ctok = Tok{EOE};
    return;
  }
  while (isspace(E.s[E.ptr])) {
    ++E.ptr;
  }  // skip whitespace

  char c = E.s[E.ptr++];
  switch (c) {
    case '+':
    case '-':
    case '/':
    case '*':
    case '^':
    case '(':
    case ')': {
      ctok = Tok{c, string(1, c)};
      return;
    }
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9': {
      int spos = E.ptr - 1;
      while (E.ptr < E.s.length() && isdigit(E.s[E.ptr])) {
        ++E.ptr;
      }
      ctok = Tok{NUM, E.s.substr(spos, E.s.length())};
      return;
    }
    default: {
      if (c >= 33 && c <= 126) {
        cout << "Error: unexpected char: " << c << '\n';
      } else {
        cout << "Error: unexpected char: " << (int)c << '\n';
      }
      ctok = Tok{TokKind::ERROR};
      return;
    }
  }
}

tuple<float, bool> expr(Expr& E);

tuple<float, bool> fact(Expr& E) {
  lex(E);
  switch (ctok.kind) {
    case NUM:
      return {stof(ctok.text), true};
    case '(': {
      auto [R, ok] = expr(E);
      if (!ok) return {0, false};
      if (ctok.kind != ')') {
        cout << "Error: expected closing ')'\n";
        return {0, false};
      }
      return {R, true};
    }
    default:
      cout << "Error: expected valid expression\n";
      return {0, false};
  }
}

tuple<float, bool> apply(float LHS, float RHS, int op) {
  switch (op) {
    case '+':
      return {LHS + RHS, true};
    case '-':
      return {LHS - RHS, true};
    case '/':
      // Might be more correct to do epsilon check
      // here :)
      if (RHS == 0) {
        cout << "Tried to divide by zero\n";
        return {0, false};
      }
      return {LHS / RHS, true};
    case '*':
      return {LHS * RHS, true};
    case '^':
      return {pow(LHS, RHS), true};
    default:
      assert("unreachable");
      break;
  }
  return {0, false};
}

tuple<float, bool> expr(Expr& E) {
  auto [res, ok] = fact(E);
  if (!ok) return {0, false};
  float LHS = res;

  struct Back {
    Tok Op;
    float R;
  };

  lex(E);
  Tok Op = ctok;
  if (Op.kind == ERROR) return {0, false};

  stack<Back> OpStack;
  while (BinOpPrec.find(Op.kind) != BinOpPrec.end()) {
    auto [res, ok] = fact(E);
    if (!ok) return {0, false};
    float RHS = res;

    lex(E);
    Tok NextOp = ctok;
    if (NextOp.kind == ERROR) return {0, false};
    bool More = BinOpPrec.find(NextOp.kind) != BinOpPrec.end();
    if (More && BinOpPrec[NextOp.kind] > BinOpPrec[Op.kind]) {
      Back B = {Op, LHS};
      LHS = RHS;
      Op = NextOp;
      OpStack.push(B);
    } else {
      auto [L, ok] = apply(LHS, RHS, Op.kind);
      if (!ok) return {0, false};
      LHS = L;

      while (!OpStack.empty()) {
        RHS = LHS;
        Back B = OpStack.top();
        if (More && BinOpPrec[NextOp.kind] > BinOpPrec[B.Op.kind]) {
          LHS = RHS;
          Op = NextOp;
          break;
        }

        OpStack.pop();
        LHS = B.R;

        auto [L, ok] = apply(LHS, RHS, B.Op.kind);
        if (!ok) return {0, false};
        LHS = L;
      }
      Op = NextOp;
    }
  }
  return {LHS, true};
}

int main() {
  std::cout << R"(
Basic C++ Calculator
________________________
Enter an expression to evaluate.
Note: Operators +, -, *, / and ^ available. ctrl+c to exit.
)" << std::endl;
  while (true) {
    cout << ">>> ";
    string s;
    std::getline(cin, s);
    if (s.empty()) {
      cout << "No input\n";
      continue;
    }

    Expr E = {s};
    auto [res, ok] = expr(E);
    if (!ok) continue;

    cout << res << '\n';
  }
}
