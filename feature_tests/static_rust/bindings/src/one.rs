use super::Two;
pub struct One;

impl One {
    fn transitivity(hold : One, nohold : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_transitivity(hold, nohold) }
    }

    fn cycle(hold : Two, nohold : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_cycle(hold, nohold) }
    }

    fn many_dependents(a : One, b : One, c : Two, d : Two, nohold : Two) -> One {
            // TODO: writeable conversions.
        unsafe { One_many_dependents(a, b, c, d, nohold) }
    }

    fn return_outlives_param(hold : Two, nohold : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_return_outlives_param(hold, nohold) }
    }

    fn diamond_top(top : One, left : One, right : One, bottom : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_diamond_top(top, left, right, bottom) }
    }

    fn diamond_left(top : One, left : One, right : One, bottom : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_diamond_left(top, left, right, bottom) }
    }

    fn diamond_right(top : One, left : One, right : One, bottom : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_diamond_right(top, left, right, bottom) }
    }

    fn diamond_bottom(top : One, left : One, right : One, bottom : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_diamond_bottom(top, left, right, bottom) }
    }

    fn diamond_and_nested_types(a : One, b : One, c : One, d : One, nohold : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_diamond_and_nested_types(a, b, c, d, nohold) }
    }

    fn implicit_bounds(explicit_hold : One, implicit_hold : One, nohold : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_implicit_bounds(explicit_hold, implicit_hold, nohold) }
    }

    fn implicit_bounds_deep(explicit_ : One, implicit_1 : One, implicit_2 : One, nohold : One) -> One {
            // TODO: writeable conversions.
        unsafe { One_implicit_bounds_deep(explicit_, implicit_1, implicit_2, nohold) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn One_transitivity(hold : One, nohold : One) -> One;

    fn One_cycle(hold : Two, nohold : One) -> One;

    fn One_many_dependents(a : One, b : One, c : Two, d : Two, nohold : Two) -> One;

    fn One_return_outlives_param(hold : Two, nohold : One) -> One;

    fn One_diamond_top(top : One, left : One, right : One, bottom : One) -> One;

    fn One_diamond_left(top : One, left : One, right : One, bottom : One) -> One;

    fn One_diamond_right(top : One, left : One, right : One, bottom : One) -> One;

    fn One_diamond_bottom(top : One, left : One, right : One, bottom : One) -> One;

    fn One_diamond_and_nested_types(a : One, b : One, c : One, d : One, nohold : One) -> One;

    fn One_implicit_bounds(explicit_hold : One, implicit_hold : One, nohold : One) -> One;

    fn One_implicit_bounds_deep(explicit_ : One, implicit_1 : One, implicit_2 : One, nohold : One) -> One;

}