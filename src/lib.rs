pub mod traits;

pub mod one_line;

#[cfg(test)]
mod test {
    mod one_line {
        use crate::one_line::*;
        use crate::traits::*;

        #[test]
        fn identity() {
            for i in 0..=20 {
                let id = OneLine::identity(i);
                assert_eq!(&id, &OneLine::new((1..=i).collect()));

                for j in 1..=i {
                    assert_eq!(j, id.apply(j));
                }
            }
        }

        #[test]
        fn apply() {
            for i in 0..=10 {
                for j in 0..=10 {
                    let p = OneLine::new((1..=i).map(|x| x + j).collect());
                    for x in 1..=i {
                        assert_eq!(x + j, p.apply(x))
                    }
                }
            }
        }
    }
}