
type
	biggest_or_smallest = ( BIGGEST, SMALLEST );

//you are not allowed to use the function power
Function power( base: LongInt; exponent: LongInt ) : LongInt;
Begin
	if exponent = 0 
	then power := 1
	else power := power(base, exponent-1) * base;
End;

// you are not allowed to use the built-in minvalue or maxvalue functions from
// pascal
Function find( var v : TIntegerDynArray; var b_o_s : biggest_or_smallest ) : LongInt;
var
	i: integer;
	result: LongInt;
Begin
	if (b_o_s = BIGGEST) then
	begin
		result := -2147483648;
		for i := 0 to length(v) do
		begin
			if result < v[i]
			then result := v[i];
		end;
	end
	else
	begin
		result := 2147483647;
		for i := 0 to length(v) do
		begin
			if result > v[i]
			then result := v[i];
		end;
		
	end;
	find := result;
End;

// you are not allowed to use the log2 function
// or the log function
Function log_base_2( n : LongInt ) : LongInt;
var
	i: integer;
	counter: LongInt;
Begin
	counter := 0;
	i := 1;
	while i <= n do
	begin
		i := i * 2;
		counter := counter + 1;
	end;
	log_base_2 := counter-1;
End;
