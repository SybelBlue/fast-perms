pub mod traits;

pub mod one_line;
pub mod involution;

#[cfg(test)]
mod test {
    mod one_line {
        use crate::{involution::{FromInvolutions, Involution}, one_line::*};
        use crate::traits::*;

        #[test]
        fn identity() {
            for i in 0..=20 {
                let id = OneLine::identity(i);
                id.validate();
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
                    let p = OneLine::new((1..=i).map(|x| (x + j - 1) % i + 1).collect());
                    p.validate();
                    for x in 1..=i {
                        assert_eq!((x + j - 1) % i + 1, p.apply(x));
                    }
                }
            }
        }

        #[test]
        fn from_involutions() {
            assert_eq!(
                OneLine::from_involutions(&Involution::new(1, 2), &Involution::new(2, 3)),
                OneLine(Box::new([2, 3, 1]))
            );
            assert_eq!(
                OneLine::from_involutions(&Involution::new(1, 2), &Involution::new(1, 3)),
                OneLine(Box::new([3, 1, 2]))
            );
            assert_eq!(OneLine::from_involutions(&Involution::new(1, 3), &Involution::new(1, 3)), OneLine::identity(3));
        }
    }
}