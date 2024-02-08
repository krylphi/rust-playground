extern crate rayon;
extern crate itertools;
use rayon::prelude::*;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

fn avg(a: f64, b: f64) -> f64 {
    (a+b)/2.0
}

fn avg_with_mass(a: f64, b: f64, amass: f64, bmass: f64) -> f64 {
    avg(a*amass, b*bmass) / (amass + bmass)
}

fn merge_two_bodies(a: Body, b: Body) -> Body {
    Body {
        x: avg_with_mass(a.x, b.x, a.mass, b.mass),
        y: avg_with_mass(a.y, b.y, a.mass, b.mass),
        z: avg_with_mass(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass,
    }
}

fn merge_all_bodies_iter(bodies: &[Body]) -> Body {
    let barry_center = bodies[0];
    bodies
        .iter()
        .skip(1)
        .fold(barry_center, |barry_center, body| {
            merge_two_bodies(barry_center, *body)
        })
}

fn merge_all_bodies_recursive(bodies: &[Body]) -> Body {
    if bodies.len() == 1 {
        return bodies[0];
    }
    let tup: Vec<_> = bodies.iter().tuples().collect();
    let mut merged: Vec<_> = tup.into_par_iter().map(|(a, b)| merge_two_bodies(*a, *b)).collect();
    if bodies.len() % 2 != 0 {
        merged.push(bodies[bodies.len() - 1]);
    }
    return merge_all_bodies_recursive(&merged)
}