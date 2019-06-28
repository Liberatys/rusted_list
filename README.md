# RustedList

[![Crates.io](https://img.shields.io/badge/crate.io-v0.0.1-orange.svg)](https://crates.io/crates/rusted_list)

A fast and always sorted list that can be used to have a chilled time with using a set of numbers that need to be sorted while using them in a list/array.


## How

The rusted_list is using a binary search algorithm to ensure that the list is sorted. This only works, because we can garante that the list is always sorted, because we only let the developer touch the methods that can insert using this method and not in an other way. 



**Insertion**

	let listing = rusted::Rusted::new();
	listing.insert(3);

	Because the size of the list is 0, we just insert the element into the vector.



	listing.insert(4);

	Because the differenz of max => 1 and min => 0 is 1, we check if the element is bigger or smaller than the current element at index 1.
	If element < vec[0]
		insert(0);
	Else
		push(element);



	listing ==> [3,4]
	listing.insert(10);


	max => 1
	min => 0

	if max - min == 1
		if element < vec[max]
			insert(max);	
		else
			insert(max - 1);
	Else
		if element > vec[max + min / 2]
			repeat(middle,max)
		else
			repeat(min,middle)



In essence, the list is using a vector as its basis. For the insertion, it is applying a recursive iteration with a binary search in order to find the index to insert the element into.


## Runtime Complexity

Because the list is using a binary search in order to insert the element, we look at a runtime of:<br>
**O(log(N))** speaking in terms of Big O.


