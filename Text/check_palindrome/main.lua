print("Word to check for palindrome: ");
local s = io.read();
if s == s.reverse(s) then
    print("That is a palindrome");
else 
    print("That is not a palindrome");
end

