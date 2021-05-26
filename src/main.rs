fn dp(n: usize, t: usize, mu: Vec<f64>, u0: Vec<f64>, delta0: Vec<f64>, p: f64) -> Vec<Vec<f64>> {
    let mut binomial: Vec<Vec<i64>> = vec![vec![0;n+1];n+1];
    binomial[0][0] = 1;
    for i in 1..=n {
        binomial[i][0] = 1;
        for j in 1..=i {
            binomial[i][j] = binomial[i-1][j-1] + binomial[i-1][j];
        }
    }
    let mut prob: Vec<f64> = vec![0.0;n+1];
    prob[0] = 1.0;
    for i in 1..=n {
        prob[i] = prob[i-1] * p;
    }
    let mut inv_prob: Vec<f64> = vec![0.0;n+1];
    inv_prob[0] = 1.0;
    for i in 1..=n {
        inv_prob[i] = inv_prob[i-1] * (1.0 - p);
    }
    let mut dp: Vec<Vec<f64>> = vec![vec![0.0;n+1];n];
    let mut cnt: Vec<Vec<i64>> = vec![vec![0;n+1];n];
    let mut res: Vec<f64> = u0.clone();
    let mut delta: Vec<f64> = u0.clone();
    let mut ans = vec![res.clone()];
    for i in 0..n {
        for k in 1..=(i+1) {
            dp[i][k] = delta0[i];
            if i > 0 {
                dp[i][k] += (cnt[i-1][k-1] as f64)*dp[i-1][k-1] + dp[i-1][k];
                cnt[i][k] = cnt[i-1][k];
                if k > 0 {
                    cnt[i][k] += cnt[i-1][k-1];
                } 
            } else {
                cnt[i][k] = 1;
            }
        }
    }
    for time in 0..=t {
        delta = vec![0.0;n];
        for i in 0..n {
            for k in 1..=n {
                delta[i] += (mu[i]*(binomial[n][k] as f64)*prob[k]*inv_prob[n-k]*dp[n-1][k]) / ((cnt[n-1][k] as f64)*(k as f64));
            }
        }
        for i in 0..n {
            res[i] += delta[i];
        }
        dp = vec![vec![0.0;n+1];n];
        cnt = vec![vec![0;n+1];n];
        for i in 0..n {
            for k in 1..=(i+1) {
                dp[i][k] = delta[i];
                if i > 0 {
                    dp[i][k] += (cnt[i-1][k-1] as f64)*dp[i-1][k-1] + dp[i-1][k];
                    cnt[i][k] = cnt[i-1][k];
                    if k > 0 {
                        cnt[i][k] += cnt[i-1][k-1];
                    } 
                } else {
                    cnt[i][k] = 1;
                }
            }
        }
        ans.push(res.clone());
    }
    ans
}

fn main() {
    let n: usize = 5;
    let mu: Vec<f64> = vec![0.3, 0.2, 0.35, 0.4, 0.3];
    let u0: Vec<f64> = vec![0.0, 0.0, 0.0, 0.1, 0.1];
    let delta0: Vec<f64> = vec![0.3, 0.0, 0.0, 0.0, 0.0];
    let p: f64 = ((n as f64).log(3.0))/(n as f64);
    let t: usize = 50;
    let res = dp(n, t, mu.clone(), u0.clone(), delta0.clone(), p);
    for time in 0..=t {
        for i in 0..n {
            print!("{} ", res[time][i]);
        }
        print!("\n");
    }
}
