inductive Palindrome : List α → Prop where
  | nil      : Palindrome []