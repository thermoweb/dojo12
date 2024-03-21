trait Food {
    fn price(&self) -> usize;
    fn name(&self) -> String;
    fn compose_name(&self) -> String;
}

trait Topping: Food {}

struct Cookie;

impl Food for Cookie {
    fn price(&self) -> usize {
        200
    }

    fn name(&self) -> String {
        "🍪".into()
    }

    fn compose_name(&self) -> String {
        self.name()
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

    fn compose_name(&self) -> String {
        self.name()
    }
}

struct Chocolate<F: Food>(F);

impl<F: Food> Food for Chocolate<F> {
    fn price(&self) -> usize {
        self.0.price() + 10
    }

    fn name(&self) -> String {
        self.0.name() + " with 🍫"
    }

    fn compose_name(&self) -> String {
        self.0.name() + " and 🍫"
    }
}

impl<T: Topping> Topping for Chocolate<T> {}

struct Nuts<F: Food>(F);

impl<F: Food> Food for Nuts<F> {
    fn price(&self) -> usize {
        self.0.price() + 20
    }

    fn name(&self) -> String {
        self.0.name() + " with 🥜"
    }

    fn compose_name(&self) -> String {
        self.0.name() + " and 🥜"
    }
}

struct Bundle {
    food: Vec<Box<dyn Food>>,
}

struct MetaBundle {
    bundles: Vec<Bundle>,
}

impl Bundle {
    pub fn new() -> Bundle {
        let vector = Vec::new();
        Bundle { food: vector }
    }

    pub fn add_food(&mut self, food: Box<dyn Food>) {
        self.food.push(food);
    }
}

impl<T: Topping> Topping for Nuts<T> {}

// tests module
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_cookie_price() {
        assert_eq!(Cookie.price(), 200)
    }

    #[test]
    fn test_cookie_name() {
        assert_eq!(Cookie.compose_name(), "🍪")
    }

    #[test]
    fn test_cupcake() {
        assert_eq!(Cupcake.price(), 100);
        assert_eq!(Cupcake.compose_name(), "🧁");
    }

    #[test]
    fn test_cupcake_with_chocolate() {
        let sut = Chocolate(Cupcake);
        assert_eq!(sut.price(), 110);
        assert_eq!(sut.compose_name(), "🧁 with 🍫");
    }

    #[test]
    fn test_cookie_with_chocolate() {
        let sut = Chocolate(Cookie);
        assert_eq!(sut.price(), 210);
        assert_eq!(sut.compose_name(), "🍪 with 🍫");
    }

    #[test]
    fn test_cookie_with_nuts() {
        let sut = Nuts(Cookie);
        assert_eq!(sut.price(), 220);
        assert_eq!(sut.compose_name(), "🍪 with 🥜");
    }

    #[test]
    fn test_cookie_with_nuts_and_chocolate() {
        let sut = Chocolate(Nuts(Cookie));
        assert_eq!(sut.price(), 230);
        assert_eq!(sut.compose_name(), "🍪 with 🥜 and 🍫");
    }

    #[test]
    fn test_cookie_with_chocolate_and_nuts() {
        let sut = Nuts(Chocolate(Cookie));
        assert_eq!(sut.price(), 230);
        assert_eq!(sut.compose_name(), "🍪 with 🍫 and 🥜");
    }

    #[test]
    fn test_bundle() {
        let mut sut = Bundle::new();
        sut.add_food(Box::new(Cookie));
    }
}
