
#include "hw1.h"

#include <limits.h>
#include <stdio.h>

unsigned int absolute_value( int x )
{
	if (x < 0)
		return x * -1;
	return x;
}

unsigned int sum_of_squares( int numbers[], int number_of_numbers )
{
	int sum = 0;
	for (int i = 0; i < number_of_numbers; i++)
	{
		sum = sum + (numbers[i] * numbers[i]);
	}
	return sum;
}

int find_biggest( int numbers[], int number_of_numbers, enum odd_or_even o_o_e)
{
	int largest = -2147483648;
	if (o_o_e == EVEN)
		for (int i = 0; i < number_of_numbers; i++)
		{
			if (numbers[i] % 2 == 0 && numbers[i] > largest)
				largest = numbers[i];
		}
	else
	{
		for (int i = 0; i < number_of_numbers; i++)
		{
			if (numbers[i] % 2 != 0 && numbers[i] > largest)
				largest = numbers[i];
		}
	}
	return largest;
}
