pub mod traits;

pub mod group_generators;

pub mod one_line;
pub mod swap;
pub mod perm64;

#[cfg(test)]
mod test {
    mod one_line {
        use crate::{swap::{FromInvolutions, Swap}, one_line::*};
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
                OneLine::from_involutions(&Swap::new(1, 2), &Swap::new(2, 3)),
                OneLine(Box::new([2, 3, 1]))
            );
            assert_eq!(
                OneLine::from_involutions(&Swap::new(1, 2), &Swap::new(1, 3)),
                OneLine(Box::new([3, 1, 2]))
            );
            assert_eq!(OneLine::from_involutions(&Swap::new(1, 3), &Swap::new(1, 3)), OneLine::identity(3));
        }
    }

    mod swap_seq {
        use crate::{swap::*, one_line::OneLine, traits::Identity};

        #[test]
        fn mul() {
            let mut swap_perm = SwapSeq::identity(4) * Swap::new(1, 2) * Swap::new(2, 3) * Swap::new(3, 4);
            assert_eq!(OneLine::new(vec![2, 3, 4, 1]), swap_perm.evaluate());
            swap_perm *= Swap::new(4, 5);
            assert_eq!(OneLine::new(vec![2, 3, 4, 5, 1]), swap_perm.evaluate());
            swap_perm.compose_left(Swap::new(4, 5));
            assert_eq!(OneLine::new(vec![2, 3, 5, 4, 1]), swap_perm.evaluate());
        }
    }
}