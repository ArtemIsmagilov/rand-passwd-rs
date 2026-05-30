# Password generator
[![Crates.io](https://img.shields.io/crates/v/rand-passwd-rs.svg)](https://crates.io/crates/rand-passwd-rs)
## Binary usage

```bash
./rand-passwd-rs 10
N(!t%7Y7S6

./rand-passwd-rs 10
Zu#NmDmLsR

./rand-passwd-rs 10
-HEH[8)Ix_
```

## Library usage

```rust
use rand_passwd_rs::rand_gen_pass;

fn main() {
  let passwd = rand_gen_pass(10);
  assert!(passwd.len() == 10);
  println!("{}", passwd)
}
```
