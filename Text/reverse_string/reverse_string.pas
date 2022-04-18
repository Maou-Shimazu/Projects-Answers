procedure reverse(const str: string);
var i, j: integer;
var reverse: string;
begin
	j := length(str);
	setlength(reverse, j);
	for i := 1 to j do
		reverse[i] := str[j - i + 1];
	writeln(reverse);
end;

var text: string;
begin
	writeln('Enter the string you want reversed...');
	readln(text);
	reverse(text);
end.
