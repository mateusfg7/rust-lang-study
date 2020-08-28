by default, all variables in Rust is imutable.
to make mutable, simple add `mut` after `let`

e.g.:

_imutable_
```rust
fn main(){
	let x = 64;
	x = 00; // return an error
}
```

_mutable_
```rust
fn main(){
 	let mut x = 64;
	x = 00; // success
}
