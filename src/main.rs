use std::vec;

trait Cake {
  fn price(&self) -> usize;
  fn name(&self) -> String {
    self
      .name_accumulator()
      .into_iter()
      .fold((0, String::new()), |(position, mut s), n| {
        match position {
          0 => s.push_str(&n),
          1 => s.push_str(&format!(" with {n}")),
          _ => s.push_str(&format!(" and {n}")),
        }
        (position + 1, s)
      })
      .1
  }
  fn name_accumulator(&self) -> Vec<String> {
    vec![self.name_constant().into()]
  }
  fn name_constant(&self) -> &'static str;
}

struct Cookie;
impl Cake for Cookie {
  fn price(&self) -> usize {
    200
  }

  fn name_constant(&self) -> &'static str {
    "🍪"
  }
}

struct Cupcake;
impl Cake for Cupcake {
  fn price(&self) -> usize {
    100
  }

  fn name_constant(&self) -> &'static str {
    "🧁"
  }
}

struct Chocolate<C: Cake>(C);
impl<C: Cake> Cake for Chocolate<C> {
  fn price(&self) -> usize {
    &self.0.price() + 10
  }

  fn name_constant(&self) -> &'static str {
    "🍫"
  }

  fn name_accumulator(&self) -> Vec<String> {
    let mut vec = self.0.name_accumulator();
    vec.push(self.name_constant().into());
    vec
  }
}

struct Nuts<C: Cake>(C);
impl<C: Cake> Cake for Nuts<C> {
  fn price(&self) -> usize {
    &self.0.price() + 20
  }

  fn name_constant(&self) -> &'static str {
    "🥜"
  }

  fn name_accumulator(&self) -> Vec<String> {
    let mut vec = self.0.name_accumulator();
    vec.push(self.name_constant().into());
    vec
  }
}

fn main() {}
// tests module
#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  #[test]
  fn test_cookie_price() {
    assert_eq!(Cookie.price(), 200)
  }

  #[test]
  fn test_cookie_name() {
    assert_eq!(Cookie.name(), "🍪")
  }

  #[test]
  fn test_cupcake() {
    assert_eq!(Cupcake.price(), 100);
    assert_eq!(Cupcake.name(), "🧁");
  }

  #[test]
  fn test_cupcake_with_chocolate() {
    let sut = Chocolate(Cupcake);
    assert_eq!(sut.price(), 110);
    assert_eq!(sut.name(), "🧁 with 🍫");
  }
  #[test]
  fn test_cookie_with_chocolate() {
    let sut = Chocolate(Cookie);
    assert_eq!(sut.price(), 210);
    assert_eq!(sut.name(), "🍪 with 🍫");
  }
  #[test]
  fn test_cookie_with_nuts() {
    let sut = Nuts(Cookie);
    assert_eq!(sut.price(), 220);
    assert_eq!(sut.name(), "🍪 with 🥜");
  }

  #[test]
  fn test_cookie_with_nuts_and_chocolate() {
    let sut = Chocolate(Nuts(Cookie));
    assert_eq!(sut.price(), 230);
    assert_eq!(sut.name(), "🍪 with 🥜 and 🍫");
  }

  #[test]
  fn test_cookie_with_chocolate_and_nuts() {
    let sut = Nuts(Chocolate(Cookie));
    assert_eq!(sut.price(), 230);
    assert_eq!(sut.name(), "🍪 with 🍫 and 🥜");
  }
}
