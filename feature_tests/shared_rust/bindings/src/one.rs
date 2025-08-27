use super::Two;
pub struct One;

impl Drop for One {
    fn drop(&mut self) {
        unsafe { One_destroy(self) }
    }
}

impl One {
    pub fn transitivity<'o, 'a, 'b, 'c, 'd, 'e, 'x, 'anon_0>(hold : &'x One<'e>, nohold : &'anon_0 One<'x>) -> Box<One><'a> {
        let ret = unsafe { One_transitivity(hold, nohold) };
        ret
    }

    pub fn cycle<'o, 'a, 'b, 'c, 'x, 'anon_0>(hold : &'anon_0 Two<'x, 'b>, nohold : &'x One<'x>) -> Box<One><'a> {
        let ret = unsafe { One_cycle(hold, nohold) };
        ret
    }

    pub fn many_dependents<'o, 'a, 'b, 'c, 'd, 'x, 'y, 'anon_0>(a : &'x One<'a>, b : &'b One<'a>, c : &'anon_0 Two<'x, 'c>, d : &'x Two<'d, 'y>, nohold : &'x Two<'x, 'y>) -> Box<One><'a> {
        let ret = unsafe { One_many_dependents(a, b, c, d, nohold) };
        ret
    }

    pub fn return_outlives_param<'o, 'short, 'long, 'anon_0>(hold : &'anon_0 Two<'long, 'short>, nohold : &'short One<'short>) -> Box<One><'long> {
        let ret = unsafe { One_return_outlives_param(hold, nohold) };
        ret
    }

    pub fn diamond_top<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'top> {
        let ret = unsafe { One_diamond_top(top, left, right, bottom) };
        ret
    }

    pub fn diamond_left<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'left> {
        let ret = unsafe { One_diamond_left(top, left, right, bottom) };
        ret
    }

    pub fn diamond_right<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'right> {
        let ret = unsafe { One_diamond_right(top, left, right, bottom) };
        ret
    }

    pub fn diamond_bottom<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'bottom> {
        let ret = unsafe { One_diamond_bottom(top, left, right, bottom) };
        ret
    }

    pub fn diamond_and_nested_types<'o, 'a, 'b, 'c, 'd, 'x, 'y, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(a : &'anon_0 One<'a>, b : &'y One<'b>, c : &'anon_1 One<'c>, d : &'anon_2 One<'d>, nohold : &'anon_3 One<'x>) -> Box<One><'a> {
        let ret = unsafe { One_diamond_and_nested_types(a, b, c, d, nohold) };
        ret
    }

    pub fn implicit_bounds<'o, 'a, 'b, 'c, 'd, 'x, 'y, 'anon_0, 'anon_1>(explicit_hold : &'d One<'x>, implicit_hold : &'anon_0 One<'x>, nohold : &'anon_1 One<'y>) -> Box<One><'a> {
        let ret = unsafe { One_implicit_bounds(explicit_hold, implicit_hold, nohold) };
        ret
    }

    pub fn implicit_bounds_deep<'o, 'a, 'b, 'c, 'd, 'x>(explicit_ : &'a One<'b>, implicit_1 : &'b One<'c>, implicit_2 : &'c One<'d>, nohold : &'x One<'x>) -> Box<One><'a> {
        let ret = unsafe { One_implicit_bounds_deep(explicit_, implicit_1, implicit_2, nohold) };
        ret
    }

}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn One_transitivity<'o, 'a, 'b, 'c, 'd, 'e, 'x, 'anon_0>(hold : &'x One<'e>, nohold : &'anon_0 One<'x>) -> Box<One><'a>;

    fn One_cycle<'o, 'a, 'b, 'c, 'x, 'anon_0>(hold : &'anon_0 Two<'x, 'b>, nohold : &'x One<'x>) -> Box<One><'a>;

    fn One_many_dependents<'o, 'a, 'b, 'c, 'd, 'x, 'y, 'anon_0>(a : &'x One<'a>, b : &'b One<'a>, c : &'anon_0 Two<'x, 'c>, d : &'x Two<'d, 'y>, nohold : &'x Two<'x, 'y>) -> Box<One><'a>;

    fn One_return_outlives_param<'o, 'short, 'long, 'anon_0>(hold : &'anon_0 Two<'long, 'short>, nohold : &'short One<'short>) -> Box<One><'long>;

    fn One_diamond_top<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'top>;

    fn One_diamond_left<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'left>;

    fn One_diamond_right<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'right>;

    fn One_diamond_bottom<'o, 'top, 'left, 'right, 'bottom, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(top : &'anon_0 One<'top>, left : &'anon_1 One<'left>, right : &'anon_2 One<'right>, bottom : &'anon_3 One<'bottom>) -> Box<One><'bottom>;

    fn One_diamond_and_nested_types<'o, 'a, 'b, 'c, 'd, 'x, 'y, 'anon_0, 'anon_1, 'anon_2, 'anon_3>(a : &'anon_0 One<'a>, b : &'y One<'b>, c : &'anon_1 One<'c>, d : &'anon_2 One<'d>, nohold : &'anon_3 One<'x>) -> Box<One><'a>;

    fn One_implicit_bounds<'o, 'a, 'b, 'c, 'd, 'x, 'y, 'anon_0, 'anon_1>(explicit_hold : &'d One<'x>, implicit_hold : &'anon_0 One<'x>, nohold : &'anon_1 One<'y>) -> Box<One><'a>;

    fn One_implicit_bounds_deep<'o, 'a, 'b, 'c, 'd, 'x>(explicit_ : &'a One<'b>, implicit_1 : &'b One<'c>, implicit_2 : &'c One<'d>, nohold : &'x One<'x>) -> Box<One><'a>;

    fn One_destroy(this : *mut One);
}