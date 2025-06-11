// Using Black-Scholes model for option pricing.

use num_traits::pow;
use rs_stats::distributions::normal_distribution::normal_cdf;
use std::f64::consts::E;

fn main() {
    let sell: f64 = 100.0; // Price of underlying asset = $100.00
    let volat: f64 = 0.25; // lower case sigma symbol = 25%
    let time: f64 = 90.0; // Time until expiration = 90 Days
    let ks: f64 = 110.0; // Strike price = $110
    let r: f64 = 0.05; // Risk-free rate = 5%

    // Risk-adjusted probability of receiving stock at expiration
    let d1: f64 = ((sell / ks).ln() + (r + (pow(volat, 2) / 2.0)) * time) / (volat * time.sqrt());
    let nd1: f64 = normal_cdf(d1, 0.0, 1.0);

    // Risk-adjusted probability of the option will be exercised.
    let d2: f64 = nd1 - volat * time.sqrt();
    let nd2: f64 = normal_cdf(d2, 0.0, 1.0);

    // Calculate fair value of the European calls and puts
    let rt: usize = (-r * time) as usize;
    let calls: f64 = nd1 * sell - ks * pow(E, rt) * nd2;
    let puts: f64 = ks * pow(E, rt) * normal_cdf(-d2, 0.0, 1.0) - sell * normal_cdf(-d1, 0.0, 1.0);

    println!("Call option price: {}", calls);
    println!("Puts option price: {}", puts);
}
