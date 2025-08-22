use super::Two;
pub struct One;

impl Drop for One {
    fn drop(&mut self) {
        unsafe { One_destroy(self) }
    }
}

impl One {
    pub fn transitivity(hold : &One, nohold : &One) -> Box<One> {
        let ret = unsafe { One_transitivity(hold, nohold) };
        ret
    }

    pub fn cycle(hold : &Two, nohold : &One) -> Box<One> {
        let ret = unsafe { One_cycle(hold, nohold) };
        ret
    }

    pub fn many_dependents(a : &One, b : &One, c : &Two, d : &Two, nohold : &Two) -> Box<One> {
        let ret = unsafe { One_many_dependents(a, b, c, d, nohold) };
        ret
    }

    pub fn return_outlives_param(hold : &Two, nohold : &One) -> Box<One> {
        let ret = unsafe { One_return_outlives_param(hold, nohold) };
        ret
    }

    pub fn diamond_top(top : &One, left : &One, right : &One, bottom : &One) -> Box<One> {
        let ret = unsafe { One_diamond_top(top, left, right, bottom) };
        ret
    }

    pub fn diamond_left(top : &One, left : &One, right : &One, bottom : &One) -> Box<One> {
        let ret = unsafe { One_diamond_left(top, left, right, bottom) };
        ret
    }

    pub fn diamond_right(top : &One, left : &One, right : &One, bottom : &One) -> Box<One> {
        let ret = unsafe { One_diamond_right(top, left, right, bottom) };
        ret
    }

    pub fn diamond_bottom(top : &One, left : &One, right : &One, bottom : &One) -> Box<One> {
        let ret = unsafe { One_diamond_bottom(top, left, right, bottom) };
        ret
    }

    pub fn diamond_and_nested_types(a : &One, b : &One, c : &One, d : &One, nohold : &One) -> Box<One> {
        let ret = unsafe { One_diamond_and_nested_types(a, b, c, d, nohold) };
        ret
    }

    pub fn implicit_bounds(explicit_hold : &One, implicit_hold : &One, nohold : &One) -> Box<One> {
        let ret = unsafe { One_implicit_bounds(explicit_hold, implicit_hold, nohold) };
        ret
    }

    pub fn implicit_bounds_deep(explicit_ : &One, implicit_1 : &One, implicit_2 : &One, nohold : &One) -> Box<One> {
        let ret = unsafe { One_implicit_bounds_deep(explicit_, implicit_1, implicit_2, nohold) };
        ret
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn One_transitivity(hold : &One, nohold : &One) -> Box<One>;

    fn One_cycle(hold : &Two, nohold : &One) -> Box<One>;

    fn One_many_dependents(a : &One, b : &One, c : &Two, d : &Two, nohold : &Two) -> Box<One>;

    fn One_return_outlives_param(hold : &Two, nohold : &One) -> Box<One>;

    fn One_diamond_top(top : &One, left : &One, right : &One, bottom : &One) -> Box<One>;

    fn One_diamond_left(top : &One, left : &One, right : &One, bottom : &One) -> Box<One>;

    fn One_diamond_right(top : &One, left : &One, right : &One, bottom : &One) -> Box<One>;

    fn One_diamond_bottom(top : &One, left : &One, right : &One, bottom : &One) -> Box<One>;

    fn One_diamond_and_nested_types(a : &One, b : &One, c : &One, d : &One, nohold : &One) -> Box<One>;

    fn One_implicit_bounds(explicit_hold : &One, implicit_hold : &One, nohold : &One) -> Box<One>;

    fn One_implicit_bounds_deep(explicit_ : &One, implicit_1 : &One, implicit_2 : &One, nohold : &One) -> Box<One>;

    fn One_destroy(this : *mut One);
}