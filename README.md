# RustedList

[![Crates.io](https://img.shields.io/badge/crate.io-v0.0.1-orange.svg)](https://crates.io/crates/rusted_list)

The rusted-list is an all time sorted list that is keeping track of its values based on their relative position to the center/middle of the list/vector.

## Tracking the values

On insert of a new item into the vector, rusted-list will lookup the values for stored positions for the list values.

Example:

	Current List:

		[3,6,9]

	Stored Elements for value tracking:
		
		L: 3
		M: 6
		R: 9


	Value to insert:
 		
		7

	
	Iterate over stored value ranges and find ranges that encapsulate the value that is to be inserted into the lsit-
	
		Left-Cap = 6
		Right-Cap = 9



	Iterate over elements between L-Cap && R-Cap and find the position to insert the value to.

	For i in range(index(Left-Cap) , index(Right-Cap)){
		if newValue >= vec[i]{
			vec.insert(i + 1,newValue);	
		}
	}


	If needed, we will update the stored values for left , middle and right if their possition has been overwritten.


	The list will at the moment only track the values for center, left and right. This could/should be changed
	in a more advanced version of the rusted_list;

	
		

By keeping track of some "tower" elements in the list, we can insert the value easily without having to iterate over all he value that are in the list. 


## Big-O test


After the implementation I will calculate the Big-O notation for the different operations on the list and will compare it to other implementations of an ever lasting sorted list.








