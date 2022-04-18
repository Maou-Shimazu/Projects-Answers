def reverse : IO String := do {
    IO.println s!"String to reverse: "
    let stdin <- IO.getStdin
    let input <- IO.FS.Stream.getLine stdin;
    pure s!"\n{input}"
}

def main : IO Unit := do {
    let rev <- reverse
    IO.println rev
}