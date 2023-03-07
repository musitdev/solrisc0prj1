# hashmap algorithm

* prover stores hints in the form of state that the zk env reads
* each get on the prover side needs to be written to zk_context
* the map state itself needs to be synced before the first get following an insert
	* insert -> insert doesn't need to sync state [NO]
	* get -> get doesn't need to sync state [NO]
	* get -> insert switch doesn't need to sync state [NO]
	* insert -> get switch needs to sync state [YES]


* the hint state consists of
 * `store_array_snaps: Vec<(K,V)>` -> this is a sorted array consisting of a tuple of keys and values
 * `store_array_sort_proofs: Vec<usize>` -> this is the index array for the above sorted array that can be used for proof for corrector sort, membership check etc 
 * `Vec<IndexProof>` 
  
```
enum IndexProof {
	E(usize),
	NE(i32, i32),
	SWITCH
}
```
  
* for each get, the prover writes the index from the above array into this vector E(usize)
* for each non-existent get, the prover writes 2 consecutvie indices as proof of non existence. since the kv vector is sorted, this can be taken as proof that an element doesn't exist  NE(i32,i32) 
* for each switch from insert -> get, the prover writes SWITCH followed by the "store_array_snaps" Vec<(K,V)> and "store_array_sort_proofs" Vec<usize>

on the zk side,
put adds each (K,V) to an `original_input_array`
for each get
* if the get is of type SWITCH, get reads the next element from the zk context to update the state. the 2 hint state arrays from the prover "store_array_snaps", "store_array_sort_proofs" can be validated using "original_input_array"
* if the get is of type E(usize), then the index is looked up from `store_array_snaps` and returned
* if the get is of type NE(i32, i32), then non-existence checks are performed on the `store_array_snaps` and `None` is returned
* if a fresh get is not of type `IndexProof` then the code panics (by design)


## example 1
```
insert(9,900)
insert(6,600)
insert(5,500)
```

since there are no gets, we don't need to save any hints

## example 2
```
insert(9,900)
insert(6,600)
insert(5,500)
get(5)
get(5)
get(5)
get(9)
get(6)
```

once get(5) is reached, the arrays need to be written to hint state by prover

store_array_snaps: [(5,500), (6,600), (9,900)]
store_array_sort_proofs: [2,1,0]

SWITCH, (store_array_snaps,store_array_sort_proofs), E(0), E(0), E(0), E(2), E(1)


## example 3
```
insert(9,900)
insert(6,600)
insert(5,500)
get(5)
get(5)
get(5)
get(9)
get(6)
get(8) -> non existent
get(3) -> non existent
insert(3,300)
insert(8,800)
get(1) -> non existent
get(3)
get(5)
get(8)
```


### prover side for example 3

NOTE: when storing the store_array_snaps, we will ignore store_array_sort_proofs to make it cleaner to look at. we can assume that a sorted array is validatable
```
insert(9,900) -> []
insert(6,600) -> []
insert(5,500) -> []

get(5) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0)
get(5) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0)
get(5) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0)
get(9) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2)
get(6) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1)
get(8) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2)
get(3) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), NE(-1,0)

insert(3,300) -> SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), NE(-1,0)
insert(8,800) -> SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), NE(-1,0)

get(1) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), NE(-1,0), SWITCH, [(3,300),(5,500),(6,600),(8,800),(9,900)], NE(-1,0)

get(3) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), NE(-1,0), SWITCH, [(3,300),(5,500),(6,600),(8,800),(9,900)], NE(-1,0), E(0)

get(5) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), NE(-1,0), SWITCH, [(3,300),(5,500),(6,600),(8,800),(9,900)], NE(-1,0), E(0), E(1)

get(8) ->  SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), NE(-1,0), SWITCH, [(3,300),(5,500),(6,600),(8,800),(9,900)], NE(-1,0), E(0), E(1), E(3)
```

### zk side for example 3

NOTE: as explained above in the prover side, store_array_sort_proofs is also included with store_array_snaps, but we're not listing that out for convenience

ZKCONTEXT / HINTS when zkenv begins execution

```
SWITCH, [(5,500),(6,600),(9,900)], E(0), E(0), E(0), E(2), E(1), NE(1,2), 
NE(-1,0), SWITCH, [(3,300),(5,500),(6,600),(8,800),(9,900)], NE(-1,0), E(0), E(1), E(3)
```

```
insert(9,900) -> original_input_array [(9,900)]
insert(6,600) -> original_input_array [(9,900),(6,600)]
insert(5,500) -> original_input_array [(9,900),(6,600),(5,500)]

get(5)  
	(SWITCH), ([(5,500),(6,600),(9,900)]) -> validate [(5,500),(6,600),(9,900)] using original_input_array [(9,900),(6,600),(5,500)]
	E(0) 
	state: [(5,500),(6,600),(9,900)]
	-> returns (5,500)

state: [(5,500),(6,600),(9,900)]
get(5) -> E(0) -> returns (5,500)
get(5) -> E(0) -> returns (5,500)
get(9) -> E(2) -> returns (9,900)
get(6) -> E(1) -> returns (6,600)
get(8) -> E(1,2) -> returns None
get(3) -> E(-1,0) -> returns None

insert(3,300) -> original_input_array [(9,900),(6,600),(5,500),(3,300)]
insert(8,800) -> original_input_array [(9,900),(6,600),(5,500),(3,300),(8,800)]

get(1) -> SWITCH, [(3,300),(5,500),(6,600),(8,800),(9,900)] -> validate using original_input_array [(9,900),(6,600),(5,500),(3,300),(8,800)]
		  NE(-1,0)
		  state: [(3,300),(5,500),(6,600),(8,800),(9,900)] 
		  -> returns None

state: [(3,300),(5,500),(6,600),(8,800),(9,900)] 
get(3) -> E(0) -> returns (3,300)
get(5) -> E(1) -> returns (5,500)
get(8) -> E(3) -> returns (8,800)
```

## Assumptions
* currently only supports Map<u32, u32> This is to avoid complications with serialization and to focus on testing the core algorithm first
* does not handle duplicate inserts / overwrrites. algorithm can be extended for this, but early on we wanted to focus on an e2e example
