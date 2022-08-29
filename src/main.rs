use std::ops::Div;
use std::iter::Sum;
use num_traits::cast::AsPrimitive;

fn main() {
    let x: Vec<f32> = vec![1., 2., 3.];
    let y: Vec<f32> = vec![2., 3., 4.];
    println!("Mean of X: {:?}", mean::<f32, f32>(x.clone()));
    println!("Mean of Y: {:?}", mean::<f32, f32>(y.clone()));
    println!("Covariance of X and Y: {:?}", covariance(x.clone(), y.clone()))
}

fn covariance(x: Vec<f32>, y: Vec<f32>) -> f32 {
    assert!(!x.is_empty(), "Vector X can't be empty!");
    assert!(!y.is_empty(), "Vector Y can't be empty!");
    assert_eq!(x.len(), y.len(), "Vectors of covariance has to be same length!");

    let xy_mean: f32 = mean(dot(x.clone(), y.clone()));
    let x_mean: f32 = mean(x.clone());
    let y_mean: f32 = mean(y.clone());

    xy_mean - x_mean * y_mean
}

fn mean<T, R>(x: Vec<T>) -> R
    where R: Div<R, Output=R> + Sum<R> + Copy + 'static,
          T: AsPrimitive<R>,
          usize: AsPrimitive<R>
{
    assert!(!x.is_empty(), "Vector X can't be empty!");
    
    let s: R = x.iter().map(|&x| x.as_()).sum();
    let n: R = x.len().as_();
    
    s / n
    
}

fn dot(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    assert!(!x.is_empty(), "Vector X can't be empty!");
    assert!(!y.is_empty(), "Vector Y can't be empty!");
    assert_eq!(x.len(), y.len(), "Vectors of dot product has to be same length!");

    let mut z: Vec<f32> = Vec::new();
    
    for i in 1..x.len() {
        z.push(x[i] * y[i]);
    }

    z
}
