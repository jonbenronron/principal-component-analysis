use std::ops::Div;
use std::iter::Sum;
use num_traits::cast::AsPrimitive;

type TMatrix = Vec<Vec<f32>>;

fn main() {
    let x: TMatrix = vec![
        vec![1., 2., 3., 4.],
        vec![3., 4., 5., 6.],
        vec![7., 8., 9., 1.],
    ];
    println!("Covariance matrix for data {:#?}: {:#?}", &x, covariance_matrix(&x));
}

fn covariance(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    assert!(!x.is_empty(), "Vector X can't be empty!");
    assert!(!y.is_empty(), "Vector Y can't be empty!");
    assert_eq!(x.len(), y.len(), "Vectors of covariance has to be same length!");

    let xy_mean: f32 = mean(&dot(x, y));
    let x_mean: f32 = mean(x);
    let y_mean: f32 = mean(y);

    xy_mean - x_mean * y_mean
}

fn mean<T, R>(x: &Vec<T>) -> R
    where R: Div<R, Output=R> + Sum<R> + Copy + 'static,
          T: AsPrimitive<R>,
          usize: AsPrimitive<R>
{
    assert!(!x.is_empty(), "Vector X can't be empty!");
    
    let s: R = x.iter().map(|&x| x.as_()).sum();
    let n: R = x.len().as_();
    
    s / n
    
}

fn dot(x: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    assert!(!x.is_empty(), "Vector X can't be empty!");
    assert!(!y.is_empty(), "Vector Y can't be empty!");
    assert_eq!(x.len(), y.len(), "Vectors of dot product has to be same length!");

    let mut z: Vec<f32> = Vec::new();
    
    for i in 1..x.len() {
        z.push(x[i] * y[i]);
    }

    z
}

fn covariance_matrix (x: &TMatrix) -> TMatrix {
    assert!(!x.is_empty(), "Matrix can't be empty!");
    for (i, v) in x.iter().enumerate() {
        assert!(!v.is_empty(), "Row vector {} of matrix can't be empty!", i);
    }

    let mut c: TMatrix = Vec::new();

    for u in x {
        let mut d: Vec<f32> = Vec::new();
        for v in x {
            d.push(covariance(u, v));
        }
        c.push(d);
    }

    c
}
