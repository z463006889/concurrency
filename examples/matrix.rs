
use std::{fmt::{Debug, Display}, ops::{Add, AddAssign, Mul}};

use anyhow::{anyhow, Result};

#[derive(Debug,PartialEq)]
struct Matrix<T> 
where T:Debug
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl <T:Debug>Matrix<T> {
    fn new(data: impl Into<Vec<T>>, rows: usize, cols:usize) -> Self{
        Matrix{data:data.into(), rows, cols}
    }
}

fn main() -> Result<()>{


    Ok(())

}


fn mutiply<T>(a:Matrix<T>,b:Matrix<T>)->Result<Matrix<T>> 
where 
    T:Copy+Debug +Default+Add<Output=T> + Mul<Output=T>+AddAssign
{
    if a.cols!= b.rows{
        return Err(anyhow!("the matrix a.cols must be equal to the matrix b.rows"));
    }
    // a [1,2,3,4,5,6] 3*2    b [1,2,3,4,5,6] 2*3    
    /*
    1,2   1,2,3
    3,4   4,5,6
    5,6
     */
    let mut data = vec![T::default();a.rows*b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix{data: data, rows:a.rows, cols:b.cols})
}



#[cfg(test)]
mod matrix_text{
    use crate::Matrix;
    use super::*;
    use ndarray::arr2;
    
    #[test]
    fn matrix() {
        /**ndarray */
        let t = arr2(&[[1,2],
            [3, 4],[5,6]]);

        let o = arr2(&[[1, 2, 3],
            [4, 5, 6]]);

        let arr = t.dot(&o);
        let mut tars = Vec::new();
        for c in arr.iter(){
            tars.push(*c);
        }
        let shapx = *arr.shape().get(0).unwrap();
        let shapy = *arr.shape().get(1).unwrap();
        let tar = Matrix::<i32>::new(tars,shapx,shapy);
        /**ndarray */

        let a = vec![1,2,3,4,5,6];
        let b =vec![1,2,3,4,5,6];
        let av = Matrix::<i32>::new(a, 3, 2);
        let bv=Matrix::<i32>::new(b,2,3);
        let ret = mutiply(av,bv).unwrap();
        assert_eq!(tar,ret);

    }

}