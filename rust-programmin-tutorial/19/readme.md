the variables only can used one time, to use one more time, pass by reference `&`

```rust
{
	let name = "name";

	print_name(name);
	print_name(name); // return a error, because the variable already used
}

{
	let name = "name";
	
	print_name(&name);
	print_name(&name); // sucess
}
```
