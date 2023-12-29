#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use ndarray::{Array2, ArrayView2, indices, s};
use vecmath::{vec2_sub, vec2_add, vec2_scale};
use std::{fs, time::Instant, ops::Mul};
use itertools::Itertools;
use linfa_linalg::qr::{QRInto, QR};
use num_traits::{One, Zero};

type N = f64; 
// max int for f64: 9,007,199,254,740,992
// sample input:      297_310_270_744_292
// u64:         8,446,744,073,709,551,615

type V = [N;2];

fn det(a: V, b: V) -> N {
    a[0]*b[1]-a[1]*b[0]
}

fn check_intersect(a1: V, a2: V, v1: V, v2: V, bounds: V) -> bool {
    let det_v = det(v1, v2);
    if det_v==0.0 {return false};
    let a21 = vec2_sub(a2, a1);
    let t1 = det(a21, v2)/det_v;
    if t1<0.0 {return false;}
    let t2 = det(a21, v1)/det_v;
    if t2<0.0 {return false;}
    
    let x = vec2_add(a1, vec2_scale(v1, t1));
    x.into_iter().all(|xi| bounds[0]<=xi && xi<=bounds[1])
}

// Given a: n x 3, b: (n or 1) x 3, return c: n x 3 s.t. rows are cross product of input rows
fn array_cross<T>(a: ArrayView2<T>, b: ArrayView2<T>) -> Array2<T> 
where T: std::clone::Clone + num_traits::Num {
    let indices = [ [1, 2], [2, 0], [0, 1] ];
    let mut res = Array2::zeros(a.raw_dim());
    for (i, [ii, iii]) in indices.iter().enumerate() {
        let c =&a.slice(s![.., *ii])*&b.slice(s![.., *iii])-&a.slice(s![.., *iii])*&b.slice(s![.., *ii]);
        res.slice_mut(s![.., i]).assign(&c);
    }
    res
}

#[test]
fn test_array_cross() {
    let a = Array2::from_shape_vec([1,3], vec![2., 3., 5.]).unwrap();
    let b = Array2::from_shape_vec([1,3], vec![7., 11., 13.]).unwrap();
    let axb = array_cross(a.view(), b.view());
    assert_eq!(axb.as_slice().unwrap(), [
        3.*13.-5.*11.,
        -(2.*13.-5.*7.),
        2.*11.-3.*7.]);
}

type M = i128;

fn solution(input_s: &str, bounds: V) -> Result<[String; 2]> {
    let input: Vec<N> = input_s
        .trim_end()
        .split(['@',',', '\n'])
        .map(|s| s.trim().parse())
        .collect::<Result<_, _>>()?;
    
    
    // let m = Array2::from_shape_vec([input.len()/6, 6], input);
    let part1 = input.chunks_exact(6).tuple_combinations().filter(|(l1, l2)| 
        check_intersect([l1[0], l1[1]], [l2[0], l2[1]], [l1[3], l1[4]], [l2[3], l2[4]], bounds)
    ).count();


    // Most elegant solution to part 2 algebra I have found:
    // Let p_i and v_i denote origin and speed of first particle.
    // Parametrize our solution by un-subscripted p and v: p+v t, then we have for any i
    //  p_i + v_i t_i = p + v t_i
    // (p_i - p) + (v_i - v) t_i == 0, which implies
    // (p_i - p) x (v_i - v) == 0, or
    // p_i x v_i + v_i x p - p_i x v + p x v == 0
    //
    // Key point here: p x v is independent of i, and rest is linear in v and p.
    // By taking differences, we can get 3 linear equations for v and p per pair of inputs.
    //  v_i x p - p_i x v  == v_i x p_i

    let m = Array2::<M>::from_shape_vec([input.len()/6, 6], 
        input.iter().map(|v| *v as M).collect_vec()
    )?;
    let pi = m.slice(s![.., 0..3]);
    let vi = m.slice(s![.., 3..6]);

    //println!("pi\n{}\nvi:\n{}", pi, vi);

    let vipi = array_cross(vi, pi);
    let n3 = vipi.len();
    let n = m.shape()[0];


    let units_vectors: [Array2<M>;3 ] = [0,1,2].map(|i| {
        Array2::from_shape_vec([1,3], 
            (0..3).map(|v| if v==i {M::one()} else {M::zero()}).collect_vec()).unwrap()
    });
    //println!("Unit vectors: {:?}", units_vectors);
    // a matrix: each row corresponds to component of v_i x p - p_i * v 

    let ad_cols = 
    units_vectors.iter().map(|e| array_cross(vi, e.view()))
    .chain(
        units_vectors.iter().map(|e| array_cross(pi, ((M::zero()-M::one())*e).view()))
    )
    .map(|mc| &mc.slice(s![1..,..])-&mc.slice(s![0,..]))
    .collect_vec();

    let ad_t = Array2::from_shape_vec([6, n3-3], 
        ad_cols.iter().flat_map(|c| c.iter()).cloned().collect_vec()
    )?;

    // println!("ad:\n{}", ad_t.t());
    // we need row differences for matrix a

    let vipi_d = &vipi.slice(s![1..,..])-&vipi.slice(s![0,..]);
    let bd_t = vipi_d.to_shape([1, n3-3])?;
    
    // solve ad*uv = b by QR: (ad_t * ad_t_t) * uv = ad_t*b_t_t
    let qr_a = &ad_t.dot(&ad_t.t());
    let qr_b = &ad_t.dot(&bd_t.t());
    //println!("qra: \n{}, qrv: \n{}", qr_a, qr_b);
    //let uv = Array2::from_shape_vec([6,1], vec![M::one();6])?;

    // Solve with f64
    let fqr_a = Array2::<f64>::from_shape_vec([6,6], qr_a.iter().map(|v| *v as f64).collect_vec())?;
    let fqr_b = Array2::<f64>::from_shape_vec([6,1], qr_b.iter().map(|v| *v as f64).collect_vec())?;
    let qr = fqr_a.qr()?;
    let fuv0 = qr.solve(&fqr_b)?; // first estimate

    let uv0 = Array2::<M>::from_shape_vec([6,1], fuv0.iter().map(|v| (*v).round() as M).collect_vec())?;
    let qr_b_rem = qr_b - qr_a.dot(&uv0);

    // Solve top-left quarter for correction:
    let f3qr_b_rem = Array2::<f64>::from_shape_vec([3,1], qr_b_rem.iter().take(3).map(|v| *v as f64).collect_vec())?;
    let qr = fqr_a.slice(s![..3, ..3]).qr()?;
    let duv = qr.solve(&f3qr_b_rem)?;

    let part2 = uv0.iter().take(3).sum::<M>() + duv.iter().sum::<f64>() as M;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input, [7., 27.])?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "2");
    assert_eq!(res[1], "47x");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = solution(&input, [200000000000000., 400000000000000.])?;
        if start.elapsed().as_millis()>00 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
    Ok(())
}
