use std::cmp::Ordering;

#[derive(Eq, Debug)]
pub struct Person<'a> {
    rank: u8,
    name: &'a str,
}

impl<'a> Person<'a> {
    pub fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

impl<'a> Ord for Person<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl<'a> PartialOrd for Person<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl<'a> PartialEq for Person<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn priority_queue() {
        let mut pq: BinaryHeap<Person> = BinaryHeap::with_capacity(10);

        assert_eq!(pq.peek(), None);

        pq.push(Person {
            rank: 1,
            name: "Herman",
        });
        pq.push(Person {
            rank: 3,
            name: "Bono",
        });
        pq.push(Person {
            rank: 2,
            name: "Charles",
        });

        assert_eq!(
            pq.peek(),
            Some(&Person {
                rank: 3,
                name: "Bono",
            })
        );

        assert_eq!(
            pq.pop(),
            Some(Person {
                rank: 3,
                name: "Bono",
            })
        );
        assert_eq!(
            pq.pop(),
            Some(Person {
                rank: 2,
                name: "Charles",
            })
        );
        assert_eq!(
            pq.pop(),
            Some(Person {
                rank: 1,
                name: "Herman",
            })
        );
    }
}
