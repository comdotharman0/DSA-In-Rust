use std::cmp::{Eq, PartialOrd};

pub struct BubbleSort;
impl BubbleSort {
    pub fn new() -> Self {
        BubbleSort
    }

    pub fn forloops<const N: usize, T>(mut a: [T; N]) -> [T; N]
    where
        T: PartialOrd + Copy,
    {
        let len = a.len();
        for j in 0..len {
            let mut swapped = false;
            for i in 0..len - j - 1 {
                if a[i] > a[i + 1] {
                    (a[i], a[i + 1]) = (a[i + 1], a[i]);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }

        a
    }

    pub fn whileloops<const N: usize, T>(mut a: [T; N]) -> [T; N]
    where
        T: PartialOrd + Copy,
    {
        let mut j = 0;
        let len = a.len();
        while j < len - 1 {
            let mut swapped = false;
            let mut i = 0;
            while i < len - j - 1 {
                if a[i] > a[i + 1] {
                    (a[i], a[i + 1]) = (a[i + 1], a[i]);
                    swapped = true;
                }
                i += 1;
            }
            if !swapped {
                break;
            }
            j += 1
        }

        a
    }

    pub fn loops<const N: usize, T>(mut a: [T; N]) -> [T; N]
    where
        T: PartialOrd + Copy,
    {
        let mut j = 0;
        let len = a.len();
        loop {
            if j == len - 1 {
                break;
            }
            let mut swapped = false;
            let mut i = 0;
            loop {
                if i == len - j - 1 {
                    break;
                }
                if a[i] > a[i + 1] {
                    (a[i], a[i + 1]) = (a[i + 1], a[i]);
                    swapped = true;
                }

                i += 1;
            }
            if !swapped {
                break;
            }

            j += 1
        }

        let a: [T; N] = a;
        a
    }

    pub fn recursions<const N: usize, T>(mut a: [T; N], mut i: usize, mut j: usize) -> [T; N]
    where
        T: PartialOrd + Copy,
    {
        //let N = a.len();
        if j < N - 1 {
            if i < N - j - 1 {
                if a[i] > a[i + 1] {
                    (a[i + 1], a[i]) = (a[i], a[i + 1]);
                }

                a = Self::recursions(a, i + 1, j);
            } else {
                j += 1;
                i = 0;
                a = Self::recursions(a, i, j);
            }
        } else {
        }
        a
    }
}
