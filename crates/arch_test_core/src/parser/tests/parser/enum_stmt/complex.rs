pub enum TestEnum {
    A,
    B,
    C(d, e::f, g<h, i>, (j, k)),
    D {
        a: (l, m),
        b: n::o<p::q<r, s>, t>,
        c: u,
    }
}