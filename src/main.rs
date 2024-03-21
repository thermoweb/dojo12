trait Food {
    fn price(&self) -> usize;
    fn name(&self) -> String;
}

trait Topping: Food {
    fn topping_price(&self) -> usize;
    fn topping_name(&self) -> String;
    fn topping_fullname(&self) -> String;
}

struct Cookie;

impl Food for Cookie {
    fn price(&self) -> usize {
        200
    }

    fn name(&self) -> String {
        "🍪".into()
    }
}

struct Cupcake;

impl Food for Cupcake {
    fn price(&self) -> usize {
        100
    }

    fn name(&self) -> String {
        "🧁".into()
    }
}

struct Chocolate<F: Food>(F);

impl<F: Food> Food for Chocolate<F> {
    fn price(&self) -> usize {
        self.0.price() + 10
    }

    fn name(&self) -> String {
        self.0.name() + " with 🍫".into()
    }
}

impl<T: Topping> Topping for Chocolate<T> {
    fn topping_price(&self) -> usize {
        self.price()
    }

    fn topping_name(&self) -> String {
        self.0.topping_name() + " and 🍫"
    }

    fn topping_fullname(&self) -> String {
        self.0.name() + " with 🍫"
    }
}

struct Nuts<F: Food>(F);

impl<F: Food> Food for Nuts<F> {
    fn price(&self) -> usize {
        self.0.price() + 20
    }

    fn name(&self) -> String {
        self.0.name() + " with 🥜".into()
    }
}

impl<T: Topping> Topping for Nuts<T> {
    fn topping_price(&self) -> usize {
        self.price()
    }

    fn topping_name(&self) -> String {
        self.0.name() + " and 🥜"
    }

    fn topping_fullname(&self) -> String {
        self.0.name() + " with 🥜"
    }
}

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
