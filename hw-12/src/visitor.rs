//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-1.29.1
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2018-10-01

trait HasName {
    fn name(&self) -> String;
}

trait Visitor<T> {
    fn visit(&self, visitable: &T) -> String;
}

trait Visitable<T> {
    fn accept(&self, visitor: &T) -> String;
}

/*
 * Traits implementations
 */

// Generic Visitor

struct Inspector {
    name: String,
}
impl Inspector {
    #[allow(dead_code)]
    fn new(name: &str) -> Inspector {
        Inspector {
            name: name.to_string(),
        }
    }
}
impl<T> Visitor<T> for Inspector
where
    T: HasName + Visitable<Inspector>,
{
    fn visit(&self, visitable: &T) -> String {
        format!("{} visits {}", self.name, visitable.name())
    }
}
impl HasName for Inspector {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

// First Visitable

struct Firm {
    name: String,
}
impl Firm {
    #[allow(dead_code)]
    fn new(name: &str) -> Firm {
        Firm {
            name: name.to_string(),
        }
    }
}
impl Visitable<Inspector> for Firm {
    fn accept(&self, visitor: &Inspector) -> String {
        // println!("{} accepts {}", self.name, visitor.name());
        visitor.visit(self)
    }
}
impl HasName for Firm {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

// 2nd Visitable

struct Foundation {
    name: String,
}
impl Foundation {
    #[allow(dead_code)]
    fn new(name: &str) -> Foundation {
        Foundation {
            name: name.to_string(),
        }
    }
}
impl Visitable<Inspector> for Foundation {
    fn accept(&self, visitor: &Inspector) -> String {
        // println!("{} accepts {}", self.name, visitor.name());
        visitor.visit(self)
    }
}
impl HasName for Foundation {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        let firm = Firm::new("Big firm");
        let inspector = Inspector::new("Chief");
        assert_eq!(firm.accept(&inspector), "Chief visits Big firm".to_string());
        let foundation = Foundation::new("Famous foundation");
        assert_eq!(
            foundation.accept(&inspector),
            "Chief visits Famous foundation".to_string()
        );
    }
}
