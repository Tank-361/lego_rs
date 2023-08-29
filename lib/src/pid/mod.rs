use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor};

pub struct PIDConfig {
  // immutable
  path: String,
  p: f64,
  i: f64,
  d: f64,
  // mutable
  izone: f64,
  error: f64,
}

impl PIDConfig {
  // getters for fields
  pub fn get_path(&self) -> &String { &self.path }
  pub fn get_p(&self) -> f64 { self.p }
  pub fn get_i(&self) -> f64 { self.i }
  pub fn get_d(&self) -> f64 { self.d }
  pub fn get_izone(&self) -> f64 { self.izone }
  pub fn get_error(&self) -> f64 { self.error }

  // setters for mutable values
  pub fn set_izone(&mut self, val: f64) { self.izone = val }
  pub fn set_error(&mut self, val: f64) { self.error = val }

  pub fn from(path: String, p: f64, i: f64, d: f64, izone: f64, error: f64) -> PIDConfig {
    return PIDConfig {
        path: path,
        p: p,
        i: i,
        d: d,
        izone: izone,
        error: error
    };
  }
}

pub enum System {
  LargeMotor(LargeMotor),
  MediumMotor(MediumMotor),
}

impl System {}

pub struct PIDController {
  // mutable (for values that are mutable in PIDConfig type)
  config: PIDConfig,
  target: f64,
  dt: f64,
  // immutable
  // the system that is controlled by this PID controller instance
  system: System,
  c: f64,
  finished: bool
}

impl PIDController {
  // getters
  pub fn get_config(&self) -> &PIDConfig { &self.config }
  pub fn get_system(&self) -> &System { &self.system }

  // closed loop
  pub fn closed_loop_c(&mut self) -> f64 {
      let x = self.config.p + self.config.i / self.dt + self.config.d * self.dt;
      self.c = x;
      return x; 
  }

  pub fn on_update(&mut self) {
      self.closed_loop_c();
      if self.config.izone > self.target - self.c {
          self.config.d -= 0.1;
      }

      if -self.config.error < self.target - self.c && self.target - self.c < self.config.error {
        self.finished = true;
      }
  }
}

