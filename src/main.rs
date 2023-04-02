type State = Vector6<f64>;
use ode_solvers::dopri5::*;
use ode_solvers::*;
type Time = f64;
fn main() {
    println!("Hello, world!");
}

impl ode_solvers::System<State> for KeplerOrbit {
    // Equations of motion of the system
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let r = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt();

        dy[0] = y[3];
        dy[1] = y[4];
        dy[2] = y[5];
        dy[3] = -self.mu * y[0] / r.powi(3);
        dy[4] = -self.mu * y[1] / r.powi(3);
        dy[5] = -self.mu * y[2] / r.powi(3);
    }
}
struct KeplerOrbit {
    mu: f64,
}
