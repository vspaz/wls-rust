mod models;
mod asserts;

use crate::models::wls::Wls;
use crate::asserts::asserts::assert_almost_equal;

fn main() {
    let x_points = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let y_points = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];
    let weights = vec![10.0, 1.0, 3.0, 8.0, 14.0, 21.0, 13.0];

    let wls = Wls::new(x_points, y_points, Some(weights));
    let point = wls.fit_linear_regression().unwrap();

    assert_almost_equal(1.410964913,point.get_intercept(), 0.00000001);
    assert_almost_equal( 0.321271930, point.get_slope(), 0.00000001);
}
